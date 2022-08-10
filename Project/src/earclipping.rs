fn run_triangulation_ear_clipping<S: AsRef<Ring>, C: AsRef<Ring>, P: AsRef<Path>>(
    subject_rings: &[S],
    subject_fill_rule: FillRule,
    clip_rings: &[C],
    clip_fill_rule: FillRule,
    operation: Operation,
    output_path: Option<P>,
    check_orientation: bool,
) -> IOResult<()> {
    // Start the Vatti benchmark.
    let now = Instant::now();

    // Execute the operation.
    let mut ctx = Context::with_rings(subject_rings, clip_rings);
    let mut tree = RingTree::default();

    ctx.set_check_orientation(check_orientation);
    ctx.run_ring_tree(&mut tree, subject_fill_rule, clip_fill_rule, operation);

    // Stop the benchmark and print the results.
    let elapsed = now.elapsed();

    println!(
        "Built ring tree in {:?}. Total rings: {}",
        elapsed,
        tree.all_nodes().len()
    );

    // Start the Ear Clipping benchmark.
    let now = Instant::now();

    // Perform the ear clipping triangulation.
    let mut triangulation = Vec::new();
    tree.triangulate(&mut triangulation, None);

    // Stop the benchmark and print the results.
    let elapsed = now.elapsed();

    println!(
        "Built triangulation (Ear Clipping) in {:?}. Total triangles: {}",
        elapsed,
        triangulation.len()
    );

    // Start a new SVG file if an output path has been provided.
    if let Some(output_path) = output_path {
        let mut svg = Svg::create(output_path, 100.0)?;

        // Write the output.
        svg.write_rings("#80ff9c", 0.5, "#888888", 1.0, 0.05, &triangulation)?;

        // Write the subject rings.
        svg.write_rings("none", 1.0, "#4e5cc7", 0.85, 0.15, subject_rings)?;

        // Write the clip rings.
        svg.write_rings("none", 1.0, "#c85c4b", 0.85, 0.15, clip_rings)?;

        // Close the SVG file.
        svg.close()?;
    }

    Ok(())
}