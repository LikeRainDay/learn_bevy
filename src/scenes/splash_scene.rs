use bevy::prelude::*;

use crate::scenes::loading_scene::TextureAssets;
use crate::scenes::SceneState;

pub struct SplashScenePlugin;

impl Plugin for SplashScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(SceneState::SplashScene), setup);
        // 监听按钮的点击事件 仅在当前场景下
        app.add_systems(Update, click_play_button.run_if(in_state(SceneState::SplashScene)));
    }
}

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Splash;

#[derive(Component)]
struct ChangeState(SceneState);

fn setup(mut commands: Commands, textures: Res<TextureAssets>) {
    info!("Splash");
    commands
        .spawn((
            StateScoped(SceneState::SplashScene),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            }, Splash,
        ))
        .with_children(|parent| {
            let button_colors = ButtonColors::default();
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(8.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: button_colors.normal.into(),
                    ..Default::default()
                }, button_colors, ChangeState(SceneState::GameScene),
                ))
                .with_children(|parent| {
                    parent
                        .spawn(
                            TextBundle::from_section(
                                "Play",
                                TextStyle {
                                    font_size: 40.0,
                                    color: Color::linear_rgb(0.9, 0.9, 0.9),
                                    ..Default::default()
                                },
                            ),
                        );
                });
        });
}

fn click_play_button(
    mut next_state: ResMut<NextState<SceneState>>,
    mut interaction_query: Query<
        // 查询出响应的属性信息
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
        ),
        // 查询时缩小范围, 提升查询的性能和效率(仅查询出 button 且 处于变更状态的数据)
        (Changed<Interaction>, With<Button>)>,
) {
    for mut item in &mut interaction_query {
        // 处理用户的按压逻辑
        match *item.0 {
            Interaction::Pressed => {
                // Some 是对rust中的Option类型进行解构, 如果不为空则执行代码块的逻辑， 为空则跳过. 这样设计会避免空问题。 同时需要注意尽量不用使用unwarp()进行解构,因为使用此种方法会导致程序崩溃
                if let Some(state) = item.3 {
                    next_state.set(state.0.clone())
                }
            }
            Interaction::Hovered => {
                *item.1 = item.2.hovered.into();
            }
            Interaction::None => {
                *item.1 = item.2.normal.into();
            }
        }
    }
}
