/*
 * @FilePath: llm_agent.rs
 * @Author: WangWindow 1598593280@qq.com
 * @Date: 2025-03-05 22:49:09
 * @LastEditors: WangWindow
 * @LastEditTime: 2025-03-05 22:49:11
 * 2025 by WangWindow, All Rights Reserved.
 * @Description:
 */
use rig::{
    agent::AgentBuilder,
    completion::Prompt,
    providers::deepseek::{self, DeepSeekCompletionModel},
};
use std::error::Error;

/// LLM代理封装，用于与大语言模型通信
pub struct LlmAgent {
    agent: rig::agent::Agent<DeepSeekCompletionModel>,
}

impl LlmAgent {
    /// 从环境变量创建一个新的LLM代理实例
    ///
    /// 需要设置DEEPSEEK_API_KEY环境变量
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let client = deepseek::Client::from_env();
        let deepseek_model = client.completion_model(deepseek::DEEPSEEK_CHAT);

        let agent = AgentBuilder::new(deepseek_model)
            .preamble(
                "\
                You are Gandalf the white and you will be conversing with other \
                powerful beings to discuss the fate of Middle Earth.\
                ",
            )
            .build();

        Ok(Self { agent })
    }

    /// 使用自定义提示词创建LLM代理
    pub async fn with_preamble(preamble: &str) -> Result<Self, Box<dyn Error>> {
        let client = deepseek::Client::from_env();
        let deepseek_model = client.completion_model(deepseek::DEEPSEEK_CHAT);

        let agent = AgentBuilder::new(deepseek_model).preamble(preamble).build();

        Ok(Self { agent })
    }

    /// 向LLM发送提示并获取响应
    pub async fn ask(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let response = self.agent.prompt(prompt).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_llm_agent() {
        let agent = LlmAgent::new().await.unwrap();
        let response = agent.ask("Say hello").await.unwrap();
        assert!(!response.is_empty());
    }
}
