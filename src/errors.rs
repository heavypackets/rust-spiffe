error_chain!{
    errors { 
        PEMParseError
        InvalidPath
        InvalidUri
    }

    foreign_links {
        SSL(::openssl::error::ErrorStack);
    }
}