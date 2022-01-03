use crate::{
    cell::CellState,
    field::{Action, Edge, Field, GameState},
    Gui,
};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalPosition, LogicalSize, PhysicalSize},
    event::{ElementState, Event, KeyboardInput, ModifiersState, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub fn gui(opt: Gui) -> Result<(), std::io::Error> {
    // Initialize some sensible default values.
    let width = opt.width;
    let height = opt.height;
    let mines = opt.mines;
    let mut f = Field::new(height as usize, width as usize, mines as usize);
    let mut old_field = f.clone();

    // Set up window.
    let event_loop = EventLoop::new();

    //let mut input = WinitInputHelper::new();
    let (window, p_width, p_height, mut _hidpi_factor) =
        create_window("Mine", &event_loop, width, height);

    let surface_texture = SurfaceTexture::new(p_width, p_height, &window);

    // TODO: This unwrap() must go.
    let mut pixels = Pixels::new(width, height, surface_texture).unwrap();
    //let mut paused = false;
    let field_ratio = width as f64 / height as f64;

    //let mut draw_state: Option<bool> = None;
    //let mut redraw = false;
    let mut previous_input: Option<VirtualKeyCode> = None;
    let mut modifiers = ModifiersState::empty();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(resized_physical_size),
                window_id,
            } if window_id == window.id() => {
                pixels.resize_surface(resized_physical_size.width, resized_physical_size.height);
                let width_height_ratio =
                    resized_physical_size.width as f64 / resized_physical_size.height as f64;
                if width_height_ratio > field_ratio {
                    // Window is broader than desirable for the ratio of the field size.
                    let new_size = PhysicalSize::new(
                        resized_physical_size.width,
                        (resized_physical_size.width as f64 * (1.0 / field_ratio)).round() as u32,
                    );
                    window.set_inner_size(new_size);
                } else if width_height_ratio < field_ratio {
                    // Window is taller than desirable for the ratio of the field size.
                    let new_size = PhysicalSize::new(
                        (resized_physical_size.height as f64 * (1.0 / field_ratio)).round() as u32,
                        resized_physical_size.height,
                    );
                    window.set_inner_size(new_size);
                }

                window.request_redraw()
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::ModifiersChanged(modifiers_changed) => {
                        modifiers = modifiers_changed
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(virtual_key_code),
                                ..
                            },
                        ..
                    } => {
                        match virtual_key_code {
                            // Flagging and revealing
                            VirtualKeyCode::F | VirtualKeyCode::Space => {
                                f.apply_action(Action::Flag)
                            }
                            VirtualKeyCode::R | VirtualKeyCode::Return | VirtualKeyCode::Tab => {
                                f.apply_action(Action::Reveal);

                                // If the previous input and the current input are the same, when the cell
                                // is attempted to be revealed, this is considered a double press. In that
                                // case, the neighbouring cells are to be revealed too, when possible.
                                if previous_input == Some(virtual_key_code) {
                                    f.apply_action(Action::RevealAround)
                                }
                            }

                            // Quit the application
                            VirtualKeyCode::Q => *control_flow = ControlFlow::Exit,

                            // Basic movement
                            // Shift not held.
                            code if !modifiers.shift() => match code {
                                VirtualKeyCode::Up | VirtualKeyCode::K => {
                                    f.apply_action(Action::CursorUp)
                                }
                                VirtualKeyCode::Down | VirtualKeyCode::J => {
                                    f.apply_action(Action::CursorDown)
                                }
                                VirtualKeyCode::Left | VirtualKeyCode::H => {
                                    f.apply_action(Action::CursorLeft)
                                }
                                VirtualKeyCode::Right | VirtualKeyCode::L => {
                                    f.apply_action(Action::CursorRight)
                                }

                                // Edge movements Down and Left
                                VirtualKeyCode::G => f.move_cursor_to_edge(Edge::Up),
                                VirtualKeyCode::Key0 => f.move_cursor_to_edge(Edge::Left),
                                _ => {}
                            },
                            // Shift held.
                            code if modifiers.shift() => match code {
                                // Edge movements Up, Down, Left, Right
                                VirtualKeyCode::Up | VirtualKeyCode::K => {
                                    f.apply_action(Action::CursorToEdgeUp)
                                }
                                VirtualKeyCode::Down | VirtualKeyCode::J | VirtualKeyCode::G => {
                                    f.apply_action(Action::CursorToEdgeDown)
                                }
                                VirtualKeyCode::Left | VirtualKeyCode::H => {
                                    f.apply_action(Action::CursorToEdgeLeft)
                                }
                                VirtualKeyCode::Right
                                | VirtualKeyCode::L
                                | VirtualKeyCode::Key4 => f.apply_action(Action::CursorToEdgeRight),
                                _ => {}
                            },
                            _ => {}
                        }
                        previous_input = Some(virtual_key_code)
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                // Application update code.

                match f.game_state() {
                    GameState::Won => {
                        window.set_title("You won :)");
                        f.reveal_all();
                    }
                    GameState::GameOver => {
                        window.set_title("You lost :(");
                        f.reveal_all();
                    }
                    GameState::Running => window.set_title(&format!(
                        "Mine — {} out of {} mines left",
                        f.mines_left(),
                        f.total_mines()
                    )),
                }

                if old_field != f {
                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw, in
                    // applications which do not always need to. Applications that redraw continuously
                    // can just render here instead.
                    window.request_redraw();
                }

                old_field = f.clone();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                draw(&f, pixels.get_frame());
                if pixels
                    .render()
                    .map_err(|e| format!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => (),
        }
    });
}

/// Create a window for the game.
fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
    width: u32,
    height: u32,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = width as f64;
    let height = height as f64;
    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let size = monitor.size().to_logical(hidpi_factor);
            (size.width, size.height)
        } else {
            (width, height)
        }
    };
    let scale = (monitor_height / height * 2.0 / 12.0).round().max(1.0);

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}

fn draw(field: &Field, screen: &mut [u8]) {
    //assert_eq!(screen.len(), 4 * field.cells.len());
    for (c, pix) in field.cells().iter().zip(screen.chunks_exact_mut(4)) {
        // Terminal color scheme
        /*
        let color = match c.cell_state() {
            CellState::Hidden => [0, 0, 0, 0],
            CellState::Flagged => [190, 0, 20, 0],
            CellState::Neighbours(n) => match n {
                0 => [214, 214, 214, 0],
                1 => [0, 118, 117, 0],
                2 => [76, 74, 117, 0],
                3 => [116, 0, 118, 0],
                4 => [206, 160, 113, 0],
                5 => [160, 211, 112, 0],
                6 => [160, 211, 112, 0],
                7 => [214, 95, 97, 0],
                8 => [70, 70, 70, 0],
                _ => panic!("No more than 8 neighbours should be possible in 2D minesweeper."),
            },
            CellState::RevealedMine => [90, 0, 20, 0],
        };
        */
        let color = match c.cell_state() {
            CellState::Hidden => [20, 20, 20, 0],
            CellState::Flagged => [214, 22, 63, 0],
            CellState::Neighbours(n) => match n {
                0 => [186, 186, 186, 0],
                1 => [0, 0, 255, 0],
                2 => [15, 112, 1, 0],
                3 => [251, 0, 6, 0],
                4 => [0, 0, 109, 0],
                5 => [107, 0, 2, 0],
                6 => [14, 110, 108, 0],
                7 => [30, 30, 30, 0],
                8 => [109, 109, 109, 0],
                _ => panic!("No more than 8 neighbours should be possible in 2D minesweeper."),
            },
            CellState::RevealedMine => [90, 0, 20, 0],
        };
        pix.copy_from_slice(&color);
    }

    // TODO: Redo this shit.
    let i = (field.cursor_pos_x() * 4) + (4 * (field.width() * field.cursor_pos_y()));
    screen[i..(i + 4)].copy_from_slice(&[231, 185, 3, 0])
}
