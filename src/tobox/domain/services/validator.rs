use crate::domain::models::session::SessionToken;

pub struct ValidatorService {
    firstname_max_length: usize,
    firstname_min_length: usize,
    firstname_regex: regex::Regex,
    lastname_max_length: usize,
    lastname_min_length: usize,
    lastname_regex: regex::Regex,
    username_max_length: usize,
    username_min_length: usize,
    username_regex: regex::Regex,
    password_max_length: usize,
    password_min_length: usize,
    email_max_length: usize,
    email_regex: regex::Regex,
    role_title_max_length: usize,
    role_title_min_length: usize,
    role_description_max_length: usize,
    role_description_min_length: usize,
    session_token_length: usize,
    permission_title_min_length: usize,
    permission_title_max_length: usize,
    permission_description_min_length: usize,
    permission_description_max_length: usize,
    service_title_min_length: usize,
    service_title_max_length: usize,
    service_description_min_length: usize,
    service_description_max_length: usize,
}

impl ValidatorService {

    pub fn new() -> Self {
        
        // User - - - - - - - - - - - - - - - - - - - - - - - - - - -
        
        // First name
        let firstname_max_length = 64;
        let firstname_min_length = 4;
        let firstname_regex = regex::Regex::new(r"^[a-zA-Zа-яА-Я]*$").unwrap();

        // Last name
        let lastname_max_length = 64;
        let lastname_min_length = 4;
        let lastname_regex = regex::Regex::new(r"^[a-zA-Zа-яА-Я]*$").unwrap();

        // Username
        let username_max_length = 32;
        let username_min_length = 4;
        let username_regex = regex::Regex::new(r"^[a-zA-Z0-9._]*$").unwrap();

        // Password
        let password_max_length = 32;
        let password_min_length = 8;

        // Email RFC2822
        let email_max_length = 255;
        let email_regex = regex::Regex::new(r"^([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x22([^\x0d\x22\x5c\x80-\xff]|\x5c[\x00-\x7f])*\x22)(\x2e([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x22([^\x0d\x22\x5c\x80-\xff]|\x5c[\x00-\x7f])*\x22))*\x40([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x5b([^\x0d\x5b-\x5d\x80-\xff]|\x5c[\x00-\x7f])*\x5d)(\x2e([^\x00-\x20\x22\x28\x29\x2c\x2e\x3a-\x3c\x3e\x40\x5b-\x5d\x7f-\xff]+|\x5b([^\x0d\x5b-\x5d\x80-\xff]|\x5c[\x00-\x7f])*\x5d))*$").unwrap();
        
        // Role - - - - - - - - - - - - - - - - - - - - - - - - - - -
        
        let role_title_max_length = 64;
        let role_title_min_length = 4;
        
        let role_description_max_length = 255;
        let role_description_min_length = 4;
        
        // Permission
        
        let permission_title_max_length = 64;
        let permission_title_min_length = 4;
        
        let permission_description_max_length = 255;
        let permission_description_min_length = 4;
        
        // Service
        
        let service_title_max_length = 64;
        let service_title_min_length = 4;
        
        let service_description_max_length = 255;
        let service_description_min_length = 4;
        
        // Session 
        
        let session_token_length = 128;
        
        ValidatorService {
            firstname_max_length,
            firstname_min_length,
            firstname_regex,
            lastname_max_length,
            lastname_min_length,
            lastname_regex,
            username_max_length,
            username_min_length,
            username_regex,
            password_max_length,
            password_min_length,
            email_max_length,
            email_regex,
            role_title_max_length,
            role_title_min_length,
            role_description_max_length,
            role_description_min_length,
            session_token_length,
            permission_title_max_length,
            permission_title_min_length,
            permission_description_max_length,
            permission_description_min_length,
            service_title_max_length,
            service_title_min_length,
            service_description_max_length,
            service_description_min_length,
        }
    }


    pub fn validate_username(&self, username: &str) -> Result<(), String> {

        if username.len() < self.username_min_length || username.len() > self.username_max_length {
            return Err(
                format!(
                    "Имя пользователя должно содержать от {} до {} символов", 
                    self.username_min_length, 
                    self.username_max_length
                )
            );
        }

        if !self.username_regex.is_match(username) {
            return Err(
                "Имя пользователя может содержать только буквы, \
                цифры, точки и символы подчеркивания".to_string()
            );
        }
        Ok(())
    }

    pub fn validate_email(&self, email: &str) -> Result<(), String> {

        if email.len() > self.email_max_length {
            return Err(
                format!("Email должен содержать максимум {} символов", self.email_max_length)
            );
        }

        if !self.email_regex.is_match(email) {
            return Err("Неверный формат email".to_string());
        }
        Ok(())
    }

    pub fn validate_password(&self, password: &str) -> Result<(), String> {

        if password.len() < self.password_min_length || password.len() > self.password_max_length {
            return Err(
                format!(
                    "Пароль должен содержать от {} до {} символов", 
                    self.password_min_length, 
                    self.password_max_length
                )
            );
        }

        if !password.chars().any(char::is_numeric) {
            return Err("Пароль должен содержать хотя бы одну цифру".to_string());
        }

        if !password.chars().any(char::is_alphabetic) {
            return Err("Пароль должен содержать хотя бы одну букву".to_string());
        }

        if password.chars().any(char::is_whitespace) {
            return Err("Пароль не должен содержать пробелов".to_string());
        }

        Ok(())
    }

    pub fn validate_last_name(&self, last_name: &str) -> Result<(), String> {
        if last_name.len() > self.lastname_max_length  || last_name.len() < self.lastname_min_length {
            return Err(format!(
                "Фамилия должна содержать от {} до {} символов", 
                self.lastname_min_length, 
                self.lastname_max_length
            ));
        }

        if !self.lastname_regex.is_match(last_name) {
            return Err("Фамилия должна состоять из латинских или кириллических букв".to_string());
        }

        Ok(())
    }

    pub fn validate_first_name(&self, first_name: &str) -> Result<(), String> {
        if first_name.len() > self.firstname_max_length || first_name.len() < self.firstname_min_length {
            return Err(format!(
                "Имя должно содержать от {} до {} символов", 
                self.firstname_min_length, 
                self.firstname_max_length
            ));
        }

        if !self.firstname_regex.is_match(first_name) {
            return Err("Имя должно состоять из латинских или кириллических букв".to_string());
        }

        Ok(())
    }
    
    pub fn validate_role_title(&self, title: &str) -> Result<(), String> {
        if title.len() < self.role_title_min_length || title.len() > self.role_title_max_length {
            return Err(format!(
                "Название роли должно содержать от {} до {} символов",
                self.role_title_min_length,
                self.role_title_max_length
            ));
        }
        Ok(())
    }
    
    pub fn validate_role_description(&self, description: &str) -> Result<(), String> {
        if description.len() < self.role_description_min_length || description.len() > self.role_description_max_length {
            return Err(format!(
                "Описание роли должно содержать от {} до {} символов",
                self.role_description_min_length,
                self.role_description_max_length
            ));
        }
        Ok(())
    }
    
    pub fn validate_session_token(&self, session_token: &SessionToken) -> Result<(), String> {
        if session_token.len() != self.session_token_length {
            return Err("Неверный формат токена сессии".to_string());
        }
        Ok(())
    }

    pub fn validate_page(&self, page: &u64) -> Result<(), String> {
        if *page == 0 {
            return Err("Номер страницы должен быть больше 0".to_string());
        }
        Ok(())
    }
    
    pub fn validate_per_page(&self, per_page: &u64) -> Result<(), String> {
        if *per_page == 0 {
            return Err("Количество элементов на странице должно быть больше 0".to_string());
        } else if *per_page > 100 {
            return Err("Количество элементов на странице должно быть не больше 100".to_string());
        }
        Ok(())
    }

    pub fn validate_permission_title(&self, title: &str) -> Result<(), String> {
        if title.len() < self.permission_title_min_length || title.len() > self.permission_title_max_length {
            return Err(format!(
                "Название разрешения должно содержать от {} до {} символов",
                self.permission_title_min_length,
                self.permission_title_max_length
            ));
        }
        Ok(())
    }

    pub fn validate_permission_description(&self, description: &str) -> Result<(), String> {
        if description.len() < self.permission_description_min_length || description.len() > self.permission_description_max_length {
            return Err(format!(
                "Описание разрешения должно содержать от {} до {} символов",
                self.permission_description_min_length,
                self.permission_description_max_length
            ));
        }
        Ok(())
    }

    pub fn validate_service_title(&self, title: &str) -> Result<(), String> {
        if title.len() < self.service_title_min_length || title.len() > self.service_title_max_length {
            return Err(format!(
                "Название сервиса должно содержать от {} до {} символов",
                self.service_title_min_length,
                self.service_title_max_length
            ));
        }
        Ok(())
    }

    pub fn validate_service_description(&self, description: &str) -> Result<(), String> {
        if description.len() < self.service_description_min_length || description.len() > self.service_description_max_length {
            return Err(format!(
                "Описание сервиса должно содержать от {} до {} символов",
                self.service_description_min_length,
                self.service_description_max_length
            ));
        }
        Ok(())
    }
    
}
