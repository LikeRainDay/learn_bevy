use bevy::prelude::*;

use crate::scenes::SceneState;

const MAIN_COLLECTION_WIDTH: f32 = 50.0;
const MAIN_COLLECTION_HEIGHT: f32 = 80.0;
const MAIN_COLLECTION_SPACING: f32 = 10.0;
const MAIN_COLLECTION_BOTTOM: f32 = 10.0;
const MAIN_COLLECTION_RIGHT: f32 = 10.0;

const CHILDREN_COLLECTION_WIDTH: f32 = 30.0;
const CHILDREN_COLLECTION_HEIGHT: f32 = 60.0;

const CHILDRENS: [&str; 2] = ["building", "package"];

// 关闭
#[derive(Component)]
pub struct CollectionPlacement;

// 打开
#[derive(Component)]
pub struct ExpandCollectionPlacement;

/**
主界面： 底部 右侧 功能收藏区域
 */
pub fn setup(mut commands: Commands) {
    commands
        .spawn((
            StateScoped(SceneState::GameScene),
            ButtonBundle {
                style: Style {
                    width: Val::Px(MAIN_COLLECTION_WIDTH),
                    height: Val::Px(MAIN_COLLECTION_HEIGHT),
                    bottom: Val::Px(MAIN_COLLECTION_BOTTOM),
                    right: Val::Px(MAIN_COLLECTION_RIGHT),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..Default::default()
            },
            CollectionPlacement
        ))
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "Open",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::BLACK,
                        ..Default::default()
                    },
                ));
        });

    CHILDRENS.iter().enumerate().for_each(|(index, child)| {
        commands
            .spawn((
                StateScoped(SceneState::GameScene),
                ButtonBundle {
                    style: Style {
                        width: Val::Px(CHILDREN_COLLECTION_WIDTH),
                        height: Val::Px(CHILDREN_COLLECTION_HEIGHT),
                        position_type: PositionType::Absolute,
                        right: Val::Px(MAIN_COLLECTION_RIGHT + MAIN_COLLECTION_SPACING + MAIN_COLLECTION_WIDTH + (CHILDREN_COLLECTION_WIDTH + MAIN_COLLECTION_SPACING) * index as f32),
                        bottom: Val::Px(MAIN_COLLECTION_BOTTOM),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                ExpandCollectionPlacement
            ))
            .with_children(|parent| {
                parent
                    .spawn(TextBundle::from_section(
                        *child,
                        TextStyle {
                            font_size: 20.0,
                            color: Color::BLACK,
                            ..Default::default()
                        },
                    ));
            });
    })
}

/**
点击事件内容
 */
pub fn click_event(
    mut interaction_query_with_collection: Query<(&Interaction, &mut Text), (Changed<Interaction>, With<Button>, With<CollectionPlacement>)>,
    mut interaction_query_with_expand: Query<(&Interaction, &mut Visibility), (Changed<Interaction>, With<Button>, With<ExpandCollectionPlacement>)>,
) {
    for (interaction, mut visibility) in &mut interaction_query_with_expand {
        match *interaction {
            Interaction::Pressed => {
                *visibility = match *visibility {
                    Visibility::Visible => Visibility::Hidden,
                    Visibility::Hidden => Visibility::Visible,
                    _ => Visibility::Hidden
                };
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }


    for mut item in &mut interaction_query_with_collection {
        match *item.0 {
            Interaction::Pressed => {
                let all_visible = interaction_query_with_expand.iter().all(|(_, &visibility)| visibility == Visibility::Visible);
                let text_value = match all_visible {
                    true => "Open",
                    false => "Close"
                };

                if let Some(text_content) = item.1.sections.first_mut() {
                    text_content.value = text_value.to_string();
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}