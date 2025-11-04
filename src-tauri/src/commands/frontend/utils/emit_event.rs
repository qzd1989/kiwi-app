use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub enum EmitEvent {
    UpdateExecutionTime,
    UpdateFrame,
    UpdateImageMatchingResult,
    UpdateImageMatchingInfo,
    UpdateImagesMatchingResult,
    UpdateImagesMatchingInfo,
}

impl From<EmitEvent> for &'static str {
    fn from(event: EmitEvent) -> Self {
        match event {
            EmitEvent::UpdateExecutionTime => "backend:update:execution_time",
            EmitEvent::UpdateFrame => "backend:update:frame",
            EmitEvent::UpdateImageMatchingResult => "backend:update:image_matching_result",
            EmitEvent::UpdateImageMatchingInfo => "backend:update:image_matching_info",
            EmitEvent::UpdateImagesMatchingResult => "backend:update:images_matching_result",
            EmitEvent::UpdateImagesMatchingInfo => "backend:update:images_matching_info",
        }
    }
}
