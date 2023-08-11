use std::io;
use std::fs;

use zip;

// run this code by doing the below
// cargo run Archive.zip

fn main() {
    println!("Hello, world!");
    // Exit from the actual logic which would be handled by another function in our case it's called real_main
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    // Collect command-line arguments into a vector
    let args: Vec<_> = std::env::args().collect();

    // Check if the program was provided with at least two arguments
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1; // Return an error code
    }

    // Get the filename from the command-line arguments and create a Path instance
    let fname = std::path::Path::new(&*args[1]);
    
    // Open the file with the provided filename
    let file = fs::File::open(&fname).unwrap();
    
    // Create a ZipArchive instance to work with the opened file
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // Iterate through each entry in the ZIP archive
    for i in 0..archive.len() {
        // Get the current entry (file) at the given index
        let mut file = archive.by_index(i).unwrap();

        // Get the enclosed name (path) of the file
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue, // Skip entries with no enclosed name
        };

        // Print the comment associated with the file entry
        let comment = file.comment();
        if !comment.is_empty() {
            println!("File {} comment: {}", i, comment);
        }

        // Check if the file entry ends with '/'
        if (*file.name()).ends_with('/') {  
            println!("File {} extracted to \"{}\"", i, outpath.display());
            // Create the directory specified by the outpath
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // Print information about the extracted file
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.display(), file.size());
            
            // Create parent directories if they don't exist
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            
            // Create the file and copy data from the archive to it
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Set file permissions on Unix systems if available
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    
    // Return success code (0) indicating successful completion
    0
}

// This code performs the following tasks:

// 1. Collect Command-Line Arguments: The program collects the command-line arguments using std::env::args().collect(), storing them in the args vector.

// 2. Argument Validation: It checks if there are at least two command-line arguments. If not, it prints a usage message and returns an error code of 1.

// 3. File Handling: It opens the specified file using fs::File::open() and creates a zip::ZipArchive instance to work with the opened file.

// 4. Iterating Through Archive Entries: It iterates through each entry in the ZIP archive using a loop with an index i.

// 5. Entry Handling:

    // 😌 For each entry, it retrieves the enclosed name (path) of the entry. If the enclosed name is absent, it skips the entry.
    // 😌 It checks if the entry ends with /. If so, it extracts the directory structure, creating any missing directories along the way.
    // 😌 If the entry is not a directory, it prints information about the extracted file, creates parent directories if needed, and copies data from the archive entry to the file.
    // 😌 On Unix systems (detected using cfg(unix)), it tries to set the file permissions based on the mode information from the archive entry.

// 6. Return Value: After processing all entries, the function returns a success code of 0 to indicate successful completion.

// This code essentially extracts files from a ZIP archive, creating directories and files as needed, and handling comments and file permissions where applicable. The real_main() function is designed to be a separate logic unit that can be tested and executed independently from the main() function.

//Posible improvement would be to swap out the `unwrap()` for `match` for gracefully handling errors without crashing the program!