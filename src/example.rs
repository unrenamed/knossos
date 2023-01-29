use knossos::maze::*;

fn main() {
    let maze = OrthogonalMazeBuilder::new().width(16).height(18).build();
    println!("\nRecursive backtracking");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(BinaryTree::new(Bias::NorthEast)))
        .build();
    println!("\nBinary Tree (NorthEast)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(BinaryTree::new(Bias::NorthWest)))
        .build();
    println!("\nBinary Tree (NorthWest)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(BinaryTree::new(Bias::SouthEast)))
        .build();
    println!("\nBinary Tree (SouthEast)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(BinaryTree::new(Bias::SouthWest)))
        .build();
    println!("\nBinary Tree (SouthWest)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .width(20)
        .algorithm(Box::new(Sidewinder))
        .build();
    println!("\nSidewinder");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .height(10)
        .width(15)
        .algorithm(Box::new(Kruskal))
        .build();

    println!("\nKruskal");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Newest)))
        .build();

    println!("\nGrowing tree (Newest)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Oldest)))
        .build();

    println!("\nGrowing tree (Oldest)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Random)))
        .build();

    println!("\nGrowing tree (Random)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Middle)))
        .build();

    println!("\nGrowing tree (Middle)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Newest50Random50)))
        .build();

    println!("\nGrowing tree (Newest50Random50)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Newest75Random25)))
        .build();

    println!("\nGrowing tree (Newest75Random25)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(GrowingTree::new(Method::Newest25Random75)))
        .build();

    println!("\nGrowing tree (Newest25Random75)");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(Prim::new()))
        .build();

    println!("\nPrim");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .height(10)
        .width(10)
        .algorithm(Box::new(HuntAndKill::new()))
        .build();

    println!("\nHunt & kill");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .height(25)
        .width(20)
        .algorithm(Box::new(AldousBroder))
        .build();

    println!("\nAldou-Broder");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .algorithm(Box::new(RecursiveDivision))
        .build();

    println!("\nRecursive Division");
    println!("{}", &maze);

    let maze = OrthogonalMazeBuilder::new()
        .height(15)
        .width(15)
        .algorithm(Box::new(Eller))
        .build();

    println!("\nEller");
    println!("{}", &maze);

    maze.save("output/maze.txt", Ascii).unwrap();
    maze.save("output/maze_game_map.txt", GameMap::new().span(3))
        .unwrap();
    maze.save("output/maze.png", Image::new().wall(15).passage(20))
        .unwrap();
}
