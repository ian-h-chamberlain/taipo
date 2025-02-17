use bevy::prelude::*;
use rand::{prelude::SliceRandom, thread_rng};

use crate::typing::TypingTargets;
use crate::FontHandles;
use crate::GameData;
use crate::TaipoState;
use crate::TextureHandles;
use crate::TypingTarget;
use crate::FONT_SIZE_LABEL;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_system_set(
                SystemSet::on_enter(TaipoState::MainMenu).with_system(main_menu_startup.system()),
            )
            .add_system_set(
                SystemSet::on_update(TaipoState::MainMenu)
                    .with_system(main_menu.system())
                    .with_system(button_system.system()),
            )
            .add_system_set(
                SystemSet::on_exit(TaipoState::MainMenu).with_system(main_menu_cleanup.system()),
            );
    }
}

pub struct MainMenuMarker;

#[derive(Clone)]
pub struct WordListSelection {
    label: String,
    lists: Vec<String>,
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.20, 0.20, 0.20).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn main_menu_startup(
    mut commands: Commands,
    font_handles: Res<FontHandles>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.5).into()),
            ..Default::default()
        })
        .insert(MainMenuMarker)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        //size: Size::new(Val::Percent(50.), Val::Percent(50.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        padding: Rect::all(Val::Px(10.)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.7).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    let selections = vec![
                        WordListSelection {
                            label: "Kana".to_string(),
                            lists: vec!["kana".to_string()],
                        },
                        WordListSelection {
                            label: "Kana + N5".to_string(),
                            lists: vec!["kana".to_string(), "n5kanji".to_string()],
                        },
                        WordListSelection {
                            label: "Kana + N5 + Yamanote".to_string(),
                            lists: vec![
                                "kana".to_string(),
                                "n5kanji".to_string(),
                                "yamanote".to_string(),
                            ],
                        },
                        WordListSelection {
                            label: "English".to_string(),
                            lists: vec!["english".to_string()],
                        },
                    ];
                    for selection in selections {
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(200.0), Val::Px(48.0)),
                                    margin: Rect::all(Val::Px(5.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                material: button_materials.normal.clone(),
                                ..Default::default()
                            })
                            .insert(selection.clone())
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
                                    text: Text::with_section(
                                        selection.label.clone(),
                                        TextStyle {
                                            font: font_handles.jptext.clone(),
                                            font_size: FONT_SIZE_LABEL,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                        Default::default(),
                                    ),
                                    ..Default::default()
                                });
                            });
                    }
                });
        });
}

fn main_menu() {}

fn main_menu_cleanup(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenuMarker>>) {
    for ent in main_menu_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

#[allow(clippy::type_complexity)]
fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &WordListSelection),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<TaipoState>>,
    texture_handles: Res<TextureHandles>,
    game_data_assets: Res<Assets<GameData>>,
    mut typing_targets: ResMut<TypingTargets>,
) {
    for (interaction, mut material, word_list_selection) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();

                let game_data = game_data_assets
                    .get(texture_handles.game_data.clone())
                    .unwrap();

                let mut rng = thread_rng();

                let mut possible_typing_targets: Vec<TypingTarget> = vec![];
                for list in &word_list_selection.lists {
                    possible_typing_targets.extend(game_data.word_lists[&list.to_string()].clone());
                }

                possible_typing_targets.shuffle(&mut rng);
                typing_targets.possible = possible_typing_targets.into();

                state.replace(TaipoState::Spawn).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
