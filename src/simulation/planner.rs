use crate::ai;
use super::profile::TargetProfile;

pub fn generate_ai_attack_plan(profile: &TargetProfile) -> String {
    let context = profile.summarize();
    let prompt = format!(
        "You are a penetration testing AI. Here is the target profile:\n{}\n\nGenerate a detailed attack plan including tool recommendations, sequence of actions, and logic behind each step. Output in markdown bullet points.",
        context
    );

    ai::query_llm(&prompt, &context)
}
