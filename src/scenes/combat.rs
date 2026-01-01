use tetra::Context;
use tetra::graphics::{self, Color, DrawParams, Rectangle};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::input::{self, Key};
use tetra::math::Vec2;
use rand::Rng;

use crate::game_state::GameState;
use crate::combat::CombatTurn;
use crate::defs::{SCREEN_WIDTH, SCREEN_HEIGHT, Scene};

pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    if state.fade_alpha > 0.0 {
        state.fade_alpha -= 0.02;
    }

    match state.combat_data.turn {
        CombatTurn::Menu => {
            if input::is_key_pressed(ctx, Key::Left) {
                if state.combat_data.menu_selection > 0 {
                    state.combat_data.menu_selection -= 1;
                }
            }
            if input::is_key_pressed(ctx, Key::Right) {
                if state.combat_data.menu_selection < 2 {
                    state.combat_data.menu_selection += 1;
                }
            }
            if input::is_key_pressed(ctx, Key::Z) || input::is_key_pressed(ctx, Key::Enter) || input::is_key_pressed(ctx, Key::F) {
                match state.combat_data.menu_selection {
                    0 => { // Fight
                        state.combat_data.turn = CombatTurn::Fighting;
                        state.combat_data.timer = 0.0;
                        state.combat_data.action_text = "You attacked Sans... MISS!".to_string();
                        state.combat_data.sans_shake = 10.0;
                    }
                    1 => { // Act
                        state.combat_data.turn = CombatTurn::Acting;
                        state.combat_data.action_text = "Check: Sans 1 ATK 1 DEF.\nThe easiest enemy. Can only deal 1 damage.".to_string();
                    }
                    2 => { // Mercy
                        state.combat_data.turn = CombatTurn::Mercy;
                        state.combat_data.action_text = "You spared Sans.".to_string();
                    }
                    _ => {}
                }
            }
        }
        CombatTurn::Fighting | CombatTurn::Acting | CombatTurn::Mercy => {
            if input::is_key_pressed(ctx, Key::Z) || input::is_key_pressed(ctx, Key::Enter) || input::is_key_pressed(ctx, Key::F) {
                if let CombatTurn::Mercy = state.combat_data.turn {
                    // End combat on mercy for now
                    state.scene = Scene::Desktop;
                    state.player_pos.x = 700.0; // Move player away so they don't re-trigger immediately
                } else {
                    state.combat_data.turn = CombatTurn::SansTurn;
                    state.combat_data.timer = 0.0;
                    state.combat_data.dialogue_text = "heh heh heh...".to_string();
                }
            }
        }
        CombatTurn::SansTurn => {
            state.combat_data.timer += 1.0;
            if state.combat_data.timer > 120.0 {
                state.combat_data.turn = CombatTurn::Menu;
                state.combat_data.dialogue_text = "You feel your sins crawling on your back.".to_string();
            }
        }
    }
    
    if state.combat_data.sans_shake > 0.0 {
        state.combat_data.sans_shake -= 0.5;
    }

    Ok(())
}

pub fn draw(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
    graphics::clear(ctx, Color::BLACK);

    // Draw Sans
    let shake_x = if state.combat_data.sans_shake > 0.0 {
        rand::thread_rng().gen_range(-5.0..5.0)
    } else {
        0.0
    };
    
    if let Some(sans_texture) = &state.sans_combat_texture {
        let s_width = sans_texture.width() as f32;
        let s_height = sans_texture.height() as f32;
        let s_origin = Vec2::new(s_width / 2.0, s_height / 2.0);
        
        sans_texture.draw(ctx, DrawParams::new()
            .position(Vec2::new(400.0 + shake_x, 200.0))
            .origin(s_origin)
            .scale(Vec2::new(3.0, 3.0))
        );
    }

    // Draw Box
    let box_rect = Mesh::rectangle(ctx, ShapeStyle::Stroke(4.0), Rectangle::new(50.0, 350.0, 700.0, 150.0)).unwrap();
    box_rect.draw(ctx, DrawParams::new().color(Color::WHITE));

    // Draw Text
    let text_content = match state.combat_data.turn {
        CombatTurn::Menu | CombatTurn::SansTurn => &state.combat_data.dialogue_text,
        _ => &state.combat_data.action_text,
    };
    
    let mut text = Text::new(text_content, state.font.clone());
    text.draw(ctx, DrawParams::new().position(Vec2::new(70.0, 370.0)).color(Color::WHITE));

    // Draw Buttons
    let buttons = ["FIGHT", "ACT", "MERCY"];
    let _button_width = 150.0;
    let start_x = 100.0;
    let y = 530.0;

    for (i, btn) in buttons.iter().enumerate() {
        let x = start_x + i as f32 * 200.0;
        let color = if state.combat_data.menu_selection == i && state.combat_data.turn == CombatTurn::Menu {
            Color::rgb(1.0, 1.0, 0.0)
        } else {
            Color::rgb(1.0, 0.5, 0.0) // Orange-ish
        };
        
        let mut btn_text = Text::new(*btn, state.font.clone());
        btn_text.draw(ctx, DrawParams::new().position(Vec2::new(x, y)).color(color));
        
        // Draw Heart if selected
        if state.combat_data.menu_selection == i && state.combat_data.turn == CombatTurn::Menu {
            if let Some(heart_texture) = &state.heart_texture {
                heart_texture.draw(ctx, DrawParams::new()
                    .position(Vec2::new(x - 30.0, y))
                    .scale(Vec2::new(0.1, 0.1)) // Reduced scale from 0.15 to 0.1
                );
            }
        }
    }

    // Draw HP Bar (Top Left)
    let bar_width = 150.0;
    let bar_height = 15.0;
    let bar_x = 20.0;
    let bar_y = 20.0;

    let health_bar_bg = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(bar_x, bar_y, bar_width, bar_height))?;
    health_bar_bg.draw(ctx, DrawParams::new().color(Color::rgb(0.2, 0.2, 0.2)));
    
    let hp_percent = state.combat_data.player_hp as f32 / state.combat_data.player_max_hp as f32;
    let health_fill_width = hp_percent * bar_width;
    
    if health_fill_width > 0.0 {
        let health_bar_fg = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(bar_x, bar_y, health_fill_width, bar_height))?;
        health_bar_fg.draw(ctx, DrawParams::new().color(Color::RED));
    }
    
    let hp_text = format!("HP: {:.0}%", hp_percent * 100.0);
    let mut hp_display = Text::new(hp_text, state.font.clone());
    hp_display.draw(ctx, DrawParams::new().position(Vec2::new(bar_x + bar_width + 10.0, bar_y)).color(Color::WHITE));

    // Fade In Overlay
    if state.fade_alpha > 0.0 {
        let fade_rect = Mesh::rectangle(ctx, ShapeStyle::Fill, Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)).unwrap();
        fade_rect.draw(ctx, DrawParams::new().color(Color::rgba(0.0, 0.0, 0.0, state.fade_alpha)));
    }

    Ok(())
}
