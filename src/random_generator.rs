use rand::Rng;
use std::fs;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use regex::Regex;

/// 自定义错误类型
#[derive(Debug)]
pub enum RandomGeneratorError {
    InvalidBounds,
    TooManyNumbers,
    IoError(std::io::Error),
    InvalidInputFormat,
    EmptyList,
}

impl fmt::Display for RandomGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandomGeneratorError::InvalidBounds => write!(f, "The lower bound must be less than or equal to the upper bound"),
            RandomGeneratorError::TooManyNumbers => write!(f, "The number of requested numbers exceeds the range size"),
            RandomGeneratorError::IoError(e) => write!(f, "IO Error: {}", e),
            RandomGeneratorError::InvalidInputFormat => write!(f, "Invalid input format for custom list"),
            RandomGeneratorError::EmptyList => write!(f, "Custom list cannot be empty"),
        }
    }
}

impl Error for RandomGeneratorError {}

impl From<std::io::Error> for RandomGeneratorError {
    fn from(error: std::io::Error) -> Self {
        RandomGeneratorError::IoError(error)
    }
}

/// 生成器模式
#[derive(Debug, Clone, PartialEq)]
pub enum GeneratorMode {
    Range,
    CustomList,
}

/// 随机数生成器配置
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub lower_bound: i64,
    pub upper_bound: i64,
    pub num_to_generate: usize,
    pub allow_duplicates: bool,
    pub mode: GeneratorMode,
    pub custom_list: Vec<i64>,
    pub custom_list_input: String,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            lower_bound: 0,
            upper_bound: 1024,
            num_to_generate: 1,
            allow_duplicates: false,
            mode: GeneratorMode::Range,
            custom_list: Vec::new(),
            custom_list_input: String::new(),
        }
    }
}

/// 优化后的随机数生成器
pub struct RandomGenerator {
    core_version: String,
    config: GeneratorConfig,
    generated_numbers: Vec<i64>,
    rng: rand::rngs::ThreadRng,
}

impl RandomGenerator {
    /// 创建新的随机数生成器实例
    pub fn new() -> Self {
        Self {
            core_version: "v2.0".to_string(),
            config: GeneratorConfig::default(),
            generated_numbers: Vec::new(),
            rng: rand::thread_rng(),
        }
    }

    /// 使用自定义配置创建生成器
    pub fn with_config(config: GeneratorConfig) -> Result<Self, RandomGeneratorError> {
        let mut generator = Self::new();
        generator.set_config(config)?;
        Ok(generator)
    }

    /// 设置配置
    pub fn set_config(&mut self, config: GeneratorConfig) -> Result<(), RandomGeneratorError> {
        self.validate_config(&config)?;
        self.config = config;
        Ok(())
    }

    /// 获取当前配置
    pub fn get_config(&self) -> &GeneratorConfig {
        &self.config
    }

    /// 设置下界
    pub fn set_lower_bound(&mut self, lower: i64) -> Result<(), RandomGeneratorError> {
        if lower > self.config.upper_bound {
            return Err(RandomGeneratorError::InvalidBounds);
        }
        self.config.lower_bound = lower;
        Ok(())
    }

    /// 设置上界
    pub fn set_upper_bound(&mut self, upper: i64) -> Result<(), RandomGeneratorError> {
        if upper < self.config.lower_bound {
            return Err(RandomGeneratorError::InvalidBounds);
        }
        self.config.upper_bound = upper;
        Ok(())
    }

    /// 设置生成数量
    pub fn set_num_to_generate(&mut self, num: usize) -> Result<(), RandomGeneratorError> {
        if !self.config.allow_duplicates {
            let range_size = match self.config.mode {
                GeneratorMode::Range => self.get_range_size(),
                GeneratorMode::CustomList => self.config.custom_list.len(),
            };
            if num > range_size {
                return Err(RandomGeneratorError::TooManyNumbers);
            }
        }
        self.config.num_to_generate = num;
        Ok(())
    }

    /// 设置是否允许重复
    pub fn set_allow_duplicates(&mut self, allow: bool) -> Result<(), RandomGeneratorError> {
        if !allow {
            let range_size = match self.config.mode {
                GeneratorMode::Range => self.get_range_size(),
                GeneratorMode::CustomList => self.config.custom_list.len(),
            };
            if self.config.num_to_generate > range_size {
                return Err(RandomGeneratorError::TooManyNumbers);
            }
        }
        self.config.allow_duplicates = allow;
        Ok(())
    }

    /// 获取是否允许重复
    pub fn get_allow_duplicates(&self) -> bool {
        self.config.allow_duplicates
    }

    /// 设置生成器模式
    pub fn set_mode(&mut self, mode: GeneratorMode) -> Result<(), RandomGeneratorError> {
        self.config.mode = mode;
        self.validate_config(&self.config)?;
        Ok(())
    }

    /// 获取生成器模式
    pub fn get_mode(&self) -> &GeneratorMode {
        &self.config.mode
    }

    /// 设置自定义列表输入
    pub fn set_custom_list_input(&mut self, input: String) -> Result<(), RandomGeneratorError> {
        self.config.custom_list_input = input;
        self.parse_custom_list()?;
        self.validate_config(&self.config)?;
        Ok(())
    }

    /// 获取自定义列表输入
    pub fn get_custom_list_input(&self) -> &str {
        &self.config.custom_list_input
    }

    /// 解析自定义列表输入
    fn parse_custom_list(&mut self) -> Result<(), RandomGeneratorError> {
        if self.config.custom_list_input.trim().is_empty() {
            self.config.custom_list.clear();
            return Ok(());
        }

        // 支持多种分隔符：逗号、空格、换行等
        let re = Regex::new(r"[,\s\n;]+").unwrap();
        let parts: Vec<&str> = re.split(&self.config.custom_list_input).collect();

        let mut numbers = Vec::new();
        for part in parts {
            if part.trim().is_empty() {
                continue;
            }

            match part.trim().parse::<i64>() {
                Ok(num) => numbers.push(num),
                Err(_) => return Err(RandomGeneratorError::InvalidInputFormat),
            }
        }

        self.config.custom_list = numbers;
        Ok(())
    }

    /// 生成随机数
    pub fn generate_numbers(&mut self) -> Result<(), RandomGeneratorError> {
        self.validate_config(&self.config)?;

        self.generated_numbers.clear();

        match self.config.mode {
            GeneratorMode::Range => {
                if self.config.allow_duplicates {
                    self.generate_range_with_duplicates();
                } else {
                    self.generate_range_without_duplicates();
                }
            }
            GeneratorMode::CustomList => {
                if self.config.allow_duplicates {
                    self.generate_custom_with_duplicates();
                } else {
                    self.generate_custom_without_duplicates();
                }
            }
        }

        Ok(())
    }

    /// 生成允许重复的随机数(范围模式)
    fn generate_range_with_duplicates(&mut self) {
        self.generated_numbers.reserve(self.config.num_to_generate);

        for _ in 0..self.config.num_to_generate {
            let num = self.rng.gen_range(self.config.lower_bound..=self.config.upper_bound);
            self.generated_numbers.push(num);
        }
    }

    /// 生成不允许重复的随机数(范围模式)
    fn generate_range_without_duplicates(&mut self) {
        let range_size = self.get_range_size();

        // 如果需要生成的数量接近范围大小,使用洗牌算法
        if self.config.num_to_generate as f64 > range_size as f64 * 0.5 {
            self.generate_range_by_shuffle();
        } else {
            self.generate_range_by_set();
        }
    }

    /// 使用洗牌算法生成不允许重复的随机数(范围模式)
    fn generate_range_by_shuffle(&mut self) {
        let mut all_numbers: Vec<i64> = (self.config.lower_bound..=self.config.upper_bound).collect();

        // Fisher-Yates 洗牌算法
        for i in (1..all_numbers.len()).rev() {
            let j = self.rng.gen_range(0..=i);
            all_numbers.swap(i, j);
        }

        self.generated_numbers = all_numbers.into_iter().take(self.config.num_to_generate).collect();
    }

    /// 使用集合生成不允许重复的随机数(范围模式)
    fn generate_range_by_set(&mut self) {
        let mut unique_set = HashSet::with_capacity(self.config.num_to_generate);

        while unique_set.len() < self.config.num_to_generate {
            let num = self.rng.gen_range(self.config.lower_bound..=self.config.upper_bound);
            unique_set.insert(num);
        }

        self.generated_numbers = unique_set.into_iter().collect();
    }

    /// 生成允许重复的随机数(自定义列表模式)
    fn generate_custom_with_duplicates(&mut self) {
        self.generated_numbers.reserve(self.config.num_to_generate);
        let list_len = self.config.custom_list.len();

        for _ in 0..self.config.num_to_generate {
            let index = self.rng.gen_range(0..list_len);
            self.generated_numbers.push(self.config.custom_list[index]);
        }
    }

    /// 生成不允许重复的随机数(自定义列表模式)
    fn generate_custom_without_duplicates(&mut self) {
        let list_len = self.config.custom_list.len();

        // 如果需要生成的数量接近列表大小,使用洗牌算法
        if self.config.num_to_generate as f64 > list_len as f64 * 0.5 {
            self.generate_custom_by_shuffle();
        } else {
            self.generate_custom_by_set();
        }
    }

    /// 使用洗牌算法生成不允许重复的随机数(自定义列表模式)
    fn generate_custom_by_shuffle(&mut self) {
        let mut shuffled_list = self.config.custom_list.clone();

        // Fisher-Yates 洗牌算法
        for i in (1..shuffled_list.len()).rev() {
            let j = self.rng.gen_range(0..=i);
            shuffled_list.swap(i, j);
        }

        self.generated_numbers = shuffled_list.into_iter().take(self.config.num_to_generate).collect();
    }

    /// 使用集合生成不允许重复的随机数(自定义列表模式)
    fn generate_custom_by_set(&mut self) {
        let mut unique_set = HashSet::with_capacity(self.config.num_to_generate);
        let list_len = self.config.custom_list.len();

        while unique_set.len() < self.config.num_to_generate {
            let index = self.rng.gen_range(0..list_len);
            unique_set.insert(self.config.custom_list[index]);
        }

        self.generated_numbers = unique_set.into_iter().collect();
    }

    /// 清除生成的数字
    pub fn clear_numbers(&mut self) {
        self.generated_numbers.clear();
    }

    /// 获取生成的数字
    pub fn get_numbers(&self) -> &[i64] {
        &self.generated_numbers
    }

    /// 获取生成的数字(可变引用)
    pub fn get_numbers_mut(&mut self) -> &mut Vec<i64> {
        &mut self.generated_numbers
    }

    /// 获取边界
    pub fn get_bounds(&self) -> (i64, i64) {
        (self.config.lower_bound, self.config.upper_bound)
    }

    /// 获取设置
    pub fn get_settings(&self) -> (usize, bool) {
        (self.config.num_to_generate, self.config.allow_duplicates)
    }

    /// 保存数字到文件
    pub fn save_numbers(&self, filename: &str) -> Result<(), RandomGeneratorError> {
        if self.generated_numbers.is_empty() {
            return Ok(());
        }

        let content = self.generated_numbers
            .iter()
            .map(|num| num.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        fs::write(filename, content)?;
        Ok(())
    }

    /// 从文件加载数字
    pub fn load_numbers(&mut self, filename: &str) -> Result<(), RandomGeneratorError> {
        let content = fs::read_to_string(filename)?;
        let numbers: Result<Vec<i64>, _> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.trim().parse::<i64>())
            .collect();

        match numbers {
            Ok(nums) => {
                self.generated_numbers = nums;
                Ok(())
            }
            Err(_) => Err(RandomGeneratorError::IoError(
                std::io::Error::new(std::io::ErrorKind::InvalidData, "文件格式不正确")
            ))
        }
    }

    /// 获取核心版本
    pub fn get_core_version(&self) -> &str {
        &self.core_version
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> GeneratorStats {
        GeneratorStats {
            count: self.generated_numbers.len(),
            min: self.generated_numbers.iter().min().copied(),
            max: self.generated_numbers.iter().max().copied(),
            sum: self.generated_numbers.iter().sum(),
            avg: if self.generated_numbers.is_empty() {
                0.0
            } else {
                self.generated_numbers.iter().sum::<i64>() as f64 / self.generated_numbers.len() as f64
            },
        }
    }

    /// 验证配置
    fn validate_config(&self, config: &GeneratorConfig) -> Result<(), RandomGeneratorError> {
        match config.mode {
            GeneratorMode::Range => {
                if config.lower_bound > config.upper_bound {
                    return Err(RandomGeneratorError::InvalidBounds);
                }

                if !config.allow_duplicates {
                    let range_size = self.get_range_size();
                    if config.num_to_generate > range_size {
                        return Err(RandomGeneratorError::TooManyNumbers);
                    }
                }
            }
            GeneratorMode::CustomList => {
                if config.custom_list.is_empty() {
                    return Err(RandomGeneratorError::EmptyList);
                }

                if !config.allow_duplicates && config.num_to_generate > config.custom_list.len() {
                    return Err(RandomGeneratorError::TooManyNumbers);
                }
            }
        }

        Ok(())
    }

    /// 获取范围大小
    fn get_range_size(&self) -> usize {
        (self.config.upper_bound - self.config.lower_bound + 1) as usize
    }
}

/// 统计信息
#[derive(Debug)]
pub struct GeneratorStats {
    pub count: usize,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub sum: i64,
    pub avg: f64,
}

impl Default for RandomGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_generation() {
        let mut random_gen = RandomGenerator::new();
        random_gen.set_num_to_generate(5).unwrap();
        random_gen.generate_numbers().unwrap();
        assert_eq!(random_gen.get_numbers().len(), 5);
    }

    #[test]
    fn test_no_duplicates() {
        let mut random_gen = RandomGenerator::new();
        random_gen.set_num_to_generate(10).unwrap();
        random_gen.set_allow_duplicates(false).unwrap();
        random_gen.generate_numbers().unwrap();

        let numbers = random_gen.get_numbers();
        let mut unique = HashSet::new();
        for &num in numbers {
            assert!(unique.insert(num), "发现重复数字: {}", num);
        }
    }

    #[test]
    fn test_bounds_validation() {
        let mut random_gen = RandomGenerator::new();
        assert!(random_gen.set_lower_bound(100).is_err());
        assert!(random_gen.set_upper_bound(-100).is_err());
    }

    #[test]
    fn test_custom_list_generation() {
        let mut random_gen = RandomGenerator::new();
        random_gen.set_mode(GeneratorMode::CustomList).unwrap();
        random_gen.set_custom_list_input("1,2,3,4,5".to_string()).unwrap();
        random_gen.set_num_to_generate(3).unwrap();
        random_gen.generate_numbers().unwrap();

        assert_eq!(random_gen.get_numbers().len(), 3);

        let numbers = random_gen.get_numbers();
        for &num in numbers {
            assert!(num >= 1 && num <= 5, "数字 {} 不在自定义列表中", num);
        }
    }
}