use std::collections::HashMap;

pub struct ValidatorService {
    object_name_max_length: usize,
    object_name_min_length: usize,
    object_name_regex: regex::Regex,
    
    metadata_key_max_length: usize,
    metadata_value_max_length: usize,
    
    username_max_length: usize,
    username_min_length: usize,
    username_regex: regex::Regex,
    password_max_length: usize,
    password_min_length: usize,
    
    role_title_max_length: usize,
    role_title_min_length: usize,
    role_description_max_length: usize,
    role_description_min_length: usize,
    
}

impl ValidatorService {

    pub fn new() -> Self {
        
        // User - - - - - - - - - - - - - - - - - - - - - - - - - - -
        
        // Object name
        let object_name_max_length = 256;
        let object_name_min_length = 1;
        let object_name_regex = regex::Regex::new(r"^[^/\\:?]+$").unwrap();
        
        // Object metadata
        let metadata_key_max_length = 64;
        let metadata_value_max_length = 256;
        
        // Username
        let username_max_length = 32;
        let username_min_length = 4;
        let username_regex = regex::Regex::new(r"^[a-zA-Z0-9._]*$").unwrap();

        // Password
        let password_max_length = 32;
        let password_min_length = 8;
        
        // Role - - - - - - - - - - - - - - - - - - - - - - - - - - -
        
        let role_title_max_length = 64;
        let role_title_min_length = 4;
        
        let role_description_max_length = 255;
        let role_description_min_length = 4;
        
        ValidatorService {
            object_name_max_length,
            object_name_min_length,
            object_name_regex,
            metadata_key_max_length,
            metadata_value_max_length,
            username_max_length,
            username_min_length,
            username_regex,
            password_max_length,
            password_min_length,
            role_title_max_length,
            role_title_min_length,
            role_description_max_length,
            role_description_min_length,
        }
    }


    pub fn validate_username(&self, username: &str) -> Result<(), String> {

        if username.len() < self.username_min_length || username.len() > self.username_max_length {
            return Err(
                format!(
                    "Username should be between {} and {} characters",
                    self.username_min_length, 
                    self.username_max_length
                )
            );
        }

        if !self.username_regex.is_match(username) {
            return Err(
                "Username should contain only letters, numbers, dots and underscores".to_string()
            );
        }
        Ok(())
    }

    pub fn validate_password(&self, password: &str) -> Result<(), String> {

        if password.len() < self.password_min_length || password.len() > self.password_max_length {
            return Err(
                format!(
                    "Password should be between {} and {} characters",
                    self.password_min_length, 
                    self.password_max_length
                )
            );
        }

        if !password.chars().any(char::is_numeric) {
            return Err("Password should contain at least one number".to_string());
        }

        if !password.chars().any(char::is_alphabetic) {
            return Err("Password should contain at least one letter".to_string());
        }

        if password.chars().any(char::is_whitespace) {
            return Err("Password should not contain whitespaces".to_string());
        }

        Ok(())
    }

    pub fn validate_object_name(&self, last_name: &str) -> Result<(), String> {
        if last_name.len() > self.object_name_max_length  || last_name.len() < self.object_name_min_length {
            return Err(format!(
                "Object name should be between {} and {} characters",
                self.object_name_min_length, 
                self.object_name_max_length
            ));
        }

        if !self.object_name_regex.is_match(last_name) {
            return Err("Object name should not contain special characters: [/, \\, :, ?]".to_string());
        }

        Ok(())
    }
    
    
    pub fn validate_object_metadata(&self, metadata: &HashMap<String, String>) -> Result<(), String> {
        for (key, value) in metadata.iter() {
            if key.len() > self.metadata_key_max_length {
                return Err(format!(
                    "Metadata key should be less than {} characters",
                    self.metadata_key_max_length
                ));
            }
            if value.len() > self.metadata_value_max_length {
                return Err(format!(
                    "Metadata value should be less than {} characters",
                    self.metadata_value_max_length
                ));
            }
        }
        Ok(())
    }
    
    pub fn validate_role_title(&self, title: &str) -> Result<(), String> {
        if title.len() < self.role_title_min_length || title.len() > self.role_title_max_length {
            return Err(format!(
                "Role title should be between {} and {} characters",
                self.role_title_min_length,
                self.role_title_max_length
            ));
        }
        Ok(())
    }
    
    pub fn validate_role_description(&self, description: &str) -> Result<(), String> {
        if description.len() < self.role_description_min_length || description.len() > self.role_description_max_length {
            return Err(format!(
                "Role description should be between {} and {} characters",
                self.role_description_min_length,
                self.role_description_max_length
            ));
        }
        Ok(())
    }
    

    pub fn validate_page(&self, page: &u64) -> Result<(), String> {
        if *page == 0 {
            return Err("Page number should be greater than 0".to_string());
        }
        Ok(())
    }
    
    pub fn validate_per_page(&self, per_page: &u64) -> Result<(), String> {
        if *per_page == 0 {
            return Err("Number of elements per page should be greater than 0".to_string());
        } else if *per_page > 100 {
            return Err("Number of elements per page should be less than 100".to_string());
        }
        Ok(())
    }
}
