/// 自定义验证函数类型
type CustomValidator = Box<dyn Fn(&str) -> Result<(), String>>;

/// 向导步骤定义
#[derive(Debug)]
pub struct WizardStep {
    /// 步骤 ID
    pub id: usize,

    /// 步骤名称
    pub name: String,

    /// 步骤描述
    pub description: String,

    /// 输入字段定义
    pub fields: Vec<InputField>,

    /// 是否可选
    pub optional: bool,
}

impl Clone for WizardStep {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            fields: self
                .fields
                .iter()
                .map(|f| InputField {
                    name: f.name.clone(),
                    field_type: f.field_type.clone(),
                    label: f.label.clone(),
                    help_text: f.help_text.clone(),
                    default: f.default.clone(),
                    validators: Vec::new(), // Validators cannot be cloned
                })
                .collect(),
            optional: self.optional,
        }
    }
}

/// 输入字段
pub struct InputField {
    /// 字段名称（用于存储到 data）
    pub name: String,

    /// 字段类型
    pub field_type: FieldType,

    /// 显示标签
    pub label: String,

    /// 帮助文本
    pub help_text: Option<String>,

    /// 默认值
    pub default: Option<String>,

    /// 验证规则
    pub validators: Vec<Validator>,
}

impl std::fmt::Debug for InputField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InputField")
            .field("name", &self.name)
            .field("field_type", &self.field_type)
            .field("label", &self.label)
            .field("help_text", &self.help_text)
            .field("default", &self.default)
            .field(
                "validators",
                &format!("{} validators", self.validators.len()),
            )
            .finish()
    }
}

/// 字段类型
#[derive(Debug, Clone)]
pub enum FieldType {
    /// 单行文本
    Text,

    /// 密码（掩码显示）
    Password,

    /// 多行文本
    MultilineText,

    /// 确认（是/否）
    Confirm { default: bool },

    /// 单选
    Select { options: Vec<String> },

    /// 多选
    MultiSelect { options: Vec<String> },
}

/// 验证器
pub enum Validator {
    /// 必填
    Required,

    /// 最小长度
    MinLength(usize),

    /// 最大长度
    MaxLength(usize),

    /// URL 格式
    Url,

    /// 自定义验证函数
    Custom(CustomValidator),
}
