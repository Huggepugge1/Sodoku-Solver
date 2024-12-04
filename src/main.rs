use glib::ControlFlow;
use gtk::prelude::*;
use gtk::{
    gdk, gio::File, glib, Application, ApplicationWindow, Button, ButtonsType, CssProvider, Entry,
    Frame, Grid, MessageDialog, MessageType,
};
use gtk4 as gtk;

mod cell;
mod sudoku;

use core::cell::RefCell;
use std::num::IntErrorKind;
use std::num::ParseIntError;
use std::rc::Rc;

const SQUARE_SIZE: i32 = 50;

fn get_sudoku(grid: &Grid) -> Result<sudoku::Sudoku, ParseIntError> {
    let mut cells = Vec::with_capacity(81);
    for _ in 0..81 {
        cells.push(0);
    }
    for outer_row in 0..3 {
        for outer_col in 0..3 {
            let inner_grid = match grid.child_at(outer_col, outer_row) {
                Some(grid) => match grid.downcast_ref::<Frame>() {
                    Some(frame) => match frame.child() {
                        Some(frame) => match frame.downcast_ref::<Grid>() {
                            Some(grid) => grid,
                            None => unreachable!(),
                        }
                        .clone(),
                        None => unreachable!(),
                    },
                    None => unreachable!(),
                },
                None => unreachable!(),
            };
            for inner_row in 0..3 {
                for inner_col in 0..3 {
                    let number = match inner_grid.child_at(inner_col, inner_row) {
                        Some(entry) => match entry.downcast_ref::<Entry>() {
                            Some(entry) => entry.text(),
                            None => unreachable!(),
                        },
                        None => unreachable!(),
                    };
                    cells[sudoku::Sudoku::get_position(
                        cell::CellPosition::new(outer_row as usize, outer_col as usize),
                        cell::CellPosition::new(inner_row as usize, inner_col as usize),
                    )] = match number.parse() {
                        Ok(n) => n,
                        Err(e) => {
                            let e: ParseIntError = e;
                            match e.kind() {
                                IntErrorKind::Empty => 0,
                                _ => return Err(e),
                            }
                        }
                    };
                }
            }
        }
    }
    Ok(sudoku::Sudoku::new(cells))
}

fn set_sudoku(grid: &Grid, sudoku: &sudoku::Sudoku) {
    let mut squares = Vec::with_capacity(81);
    for _ in 0..81 {
        squares.push(0);
    }
    for outer_row in 0..3 {
        for outer_col in 0..3 {
            let inner_grid = match grid.child_at(outer_col, outer_row) {
                Some(grid) => match grid.downcast_ref::<Frame>() {
                    Some(frame) => match frame.child() {
                        Some(frame) => match frame.downcast_ref::<Grid>() {
                            Some(grid) => grid,
                            None => unreachable!(),
                        }
                        .clone(),
                        None => unreachable!(),
                    },
                    None => unreachable!(),
                },
                None => unreachable!(),
            };
            for inner_row in 0..3 {
                for inner_col in 0..3 {
                    match inner_grid.child_at(inner_col, inner_row) {
                        Some(entry) => match entry.downcast_ref::<Entry>() {
                            Some(entry) => {
                                entry.set_text(
                                    &sudoku.cells[sudoku::Sudoku::get_position(
                                        cell::CellPosition::new(
                                            outer_row as usize,
                                            outer_col as usize,
                                        ),
                                        cell::CellPosition::new(
                                            inner_row as usize,
                                            inner_col as usize,
                                        ),
                                    )]
                                    .to_string(),
                                );
                            }
                            None => unreachable!(),
                        },
                        None => unreachable!(),
                    };
                }
            }
        }
    }
}

fn generate_grid() -> Grid {
    let grid = Grid::builder()
        .column_spacing(10)
        .row_spacing(10)
        .margin_top(100)
        .margin_bottom(100)
        .margin_start(100)
        .margin_end(100)
        .width_request(SQUARE_SIZE * 9)
        .height_request(SQUARE_SIZE * 9)
        .vexpand_set(false)
        .hexpand_set(false)
        .build();

    for outer_i in 0..3 {
        for outer_j in 0..3 {
            let inner_grid = Grid::builder()
                .column_spacing(5)
                .row_spacing(5)
                .vexpand_set(false)
                .hexpand_set(false)
                .build();
            for inner_i in 0..3 {
                for inner_j in 0..3 {
                    let child = Entry::builder()
                        .max_width_chars(1)
                        .width_request(SQUARE_SIZE)
                        .height_request(SQUARE_SIZE)
                        .build();

                    child.connect_changed(move |child| {
                        let text = child.text().to_string();
                        if !text.chars().all(char::is_numeric) {
                            if text.chars().nth(1) == Some(' ') {
                                child.set_text("");
                            } else if text.len() > 1 {
                                child.set_text(&text[0..1]);
                            } else {
                                child.set_text("");
                            }
                        } else if text.len() > 1 {
                            child.set_text(&text[1..])
                        }
                    });

                    inner_grid.attach(&child, inner_i, inner_j, 1, 1);
                }
            }
            let frame = Frame::builder()
                .css_classes(vec![".inner-grid"])
                .child(&inner_grid)
                .build();
            grid.attach(&frame, outer_i, outer_j, 1, 1);
        }
    }

    grid
}

fn show_err(error: &str, window: &ApplicationWindow) {
    let dialog = MessageDialog::builder()
        .transient_for(window)
        .message_type(MessageType::Error)
        .buttons(ButtonsType::Ok)
        .text(error)
        .modal(true)
        .build();

    dialog.connect_response(|dialog, _response| {
        dialog.close();
    });

    dialog.show();
}

fn solve(grid: &Grid, sudoku: &mut sudoku::Sudoku, window: &ApplicationWindow) {
    match get_sudoku(grid) {
        Ok(s) => *sudoku = s,
        Err(e) => eprintln!("{}", e),
    }
    match sudoku.solve() {
        Ok(s) => set_sudoku(&grid, &s),
        Err(e) => show_err(e, window),
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.huggepugge.sudoku_solver")
        .build();

    let mut cells = Vec::new();
    for _ in 0..81 {
        cells.push(0);
    }
    let sudoku = Rc::new(RefCell::new(sudoku::Sudoku::new(cells)));

    app.connect_activate(move |app| {
        let window = RefCell::new(
            ApplicationWindow::builder()
                .application(app)
                .title("Sudoku Solver")
                .build(),
        );
        let css_provider = CssProvider::new();
        css_provider.load_from_file(&File::for_path("grid.css"));

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let container = gtk::Box::new(gtk::Orientation::Vertical, 0);

        let grid = RefCell::new(generate_grid());
        let grid_clone = &grid.borrow_mut().clone();
        container.append(grid_clone);

        set_sudoku(&grid.borrow_mut(), &sudoku.borrow_mut());

        let sudoku_solve_button = Button::builder().label("Solve Sudoku").build();

        let sudoku_clone = sudoku.clone();
        let window_clone = window.clone();
        let grid_clone = grid.clone();
        sudoku_solve_button.connect_clicked(move |_| {
            solve(
                &grid_clone.borrow_mut(),
                &mut sudoku_clone.borrow_mut(),
                &window_clone.borrow(),
            )
        });
        container.append(&sudoku_solve_button);

        let check_soduku_button = Button::builder().label("Check Sudoku").build();

        let sudoku_clone = sudoku.clone();
        let window_clone = window.clone();
        check_soduku_button.connect_clicked(move |_| match sudoku_clone.borrow_mut().check() {
            true => {
                let dialog = MessageDialog::builder()
                    .transient_for(&window_clone.borrow_mut().clone())
                    .message_type(MessageType::Info)
                    .buttons(ButtonsType::Ok)
                    .text("Sudoku can be solved")
                    .modal(true)
                    .build();

                dialog.connect_response(|dialog, _response| {
                    dialog.close();
                });

                dialog.show();
            }
            false => show_err("Sudoku not solvable!", &window_clone.borrow_mut()),
        });
        container.append(&check_soduku_button);

        let sudoku_backprop_start = Button::builder()
            .label("Start Backpropogation Visualizer")
            .build();

        let sudoku_clone = sudoku.clone();
        let window_clone = window.clone();
        let grid_clone = grid.clone();
        sudoku_backprop_start.connect_clicked(move |button| {
            let sudoku_clone = sudoku_clone.clone();
            let window_clone = window_clone.clone();
            let grid_clone = grid_clone.clone();
            let visualizer = sudoku_clone.borrow_mut().visualizer;
            let name = match visualizer {
                false => "Stop Backpropogation Visualizer",
                true => "Start Backpropogation Visualizer",
            };
            button.set_label(name);
            sudoku_clone.borrow_mut().visualizer = !visualizer;
            if !visualizer {
                glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                    let sudoku = sudoku_clone.borrow_mut().clone();
                    if !sudoku.visualizer {
                        return ControlFlow::Break;
                    }
                    match sudoku_clone.borrow_mut().back_prop_next_step() {
                        Ok(()) => {
                            set_sudoku(&grid_clone.borrow_mut(), &sudoku);
                            ControlFlow::Continue
                        }
                        Err(e) => {
                            show_err(e, &window_clone.borrow_mut());
                            ControlFlow::Break
                        }
                    }
                });
            }
        });
        container.append(&sudoku_backprop_start);

        let sudoku_backprop_step = Button::builder()
            .label("Step Backpropogation Visualizer")
            .build();

        let sudoku_clone = sudoku.clone();
        let window_clone = window.clone();
        let grid_clone = grid.clone();
        sudoku_backprop_step.connect_clicked(move |_| {
            let sudoku = sudoku_clone.borrow_mut().clone();
            match sudoku_clone.borrow_mut().back_prop_next_step() {
                Ok(()) => set_sudoku(&grid_clone.borrow_mut(), &sudoku),
                Err(e) => show_err(e, &window_clone.borrow_mut()),
            }
        });
        container.append(&sudoku_backprop_step);

        window.borrow().set_child(Some(&container));

        window.borrow().present();
    });

    app.run()
}
