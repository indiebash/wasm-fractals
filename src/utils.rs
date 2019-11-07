pub fn screen_coords_to_zero_centered_cartesian_coords(screen_x: f64,
                                              screen_y: f64,
                                              translated_graph_width: f64,
                                              translated_graph_height: f64,
                                              canvas_width: f64) -> (f64, f64) {

    let scale = translated_graph_width / canvas_width;
    let scaled_x = screen_x * scale;
    let scaled_y = screen_y * scale;

    let shifted_x = scaled_x - translated_graph_width / 2.0;
    let shifted_y = translated_graph_height / 2.0 - scaled_y;

    return (shifted_x, shifted_y);
}
