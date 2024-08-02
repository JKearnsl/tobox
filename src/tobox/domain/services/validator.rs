
pub struct ValidatorService {
    box_name_max_length: usize,
    box_name_min_length: usize,
    box_name_regex: regex::Regex,
    
    object_name_max_length: usize,
    object_name_min_length: usize,
    object_name_regex: regex::Regex,
    
    object_path_max_length: usize,
    object_path_min_length: usize,
    object_path_regex: regex::Regex,
    
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
        
        // Box name
        let box_name_max_length = 64;
        let box_name_min_length = 4;
        let box_name_regex = regex::Regex::new(r"^[a-zA-Zа-яА-Я0-9]*$").unwrap();

        // Object name
        let object_name_max_length = 256;
        let object_name_min_length = 1;
        let object_name_regex = regex::Regex::new(r"^[^/]+$").unwrap();
        
        // Object path
        let object_path_max_length = 256;
        let object_path_min_length = 1;
        let object_path_regex = regex::Regex::new(r"^(/[^/\x00]+)+/?$").unwrap();
        
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
            box_name_max_length,
            box_name_min_length,
            box_name_regex,
            object_name_max_length,
            object_name_min_length,
            object_name_regex,
            object_path_max_length,
            object_path_min_length,
            object_path_regex,
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
            return Err("Object name should not contain slashes".to_string());
        }

        Ok(())
    }
    
    pub fn validate_object_path(&self, path: &str) -> Result<(), String> {
        if path.len() > self.object_path_max_length || path.len() < self.object_path_min_length {
            return Err(format!(
                "Object path should be between {} and {} characters",
                self.object_path_min_length, 
                self.object_path_max_length
            ));
        }

        if !self.object_path_regex.is_match(path) {
            return Err("Object path should be a valid unix path".to_string());
        }

        Ok(())
    }

    pub fn validate_box_name(&self, first_name: &str) -> Result<(), String> {
        if first_name.len() > self.box_name_max_length || first_name.len() < self.box_name_min_length {
            return Err(format!(
                "Box name should be between {} and {} characters",
                self.box_name_min_length, 
                self.box_name_max_length
            ));
        }

        if !self.box_name_regex.is_match(first_name) {
            return Err("Box name should contain only letters and numbers".to_string());
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
