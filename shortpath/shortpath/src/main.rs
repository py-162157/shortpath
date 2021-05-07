
struct Edge {
    start: usize,
    end: usize,
    weight: usize,
}

fn main() {
    let nodes_num = 7;
    let src = 6;
    let dst = 2;
    let mut edges = Vec::<Edge>::new();
    edges.push(Edge {start: 0, end: 1, weight:12});
    edges.push(Edge {start: 0, end: 5, weight:16});
    edges.push(Edge {start: 0, end: 6, weight:14});
    edges.push(Edge {start: 1, end: 2, weight:10});
    edges.push(Edge {start: 1, end: 5, weight:7});
    edges.push(Edge {start: 2, end: 5, weight:6});
    edges.push(Edge {start: 2, end: 4, weight:5});
    edges.push(Edge {start: 2, end: 3, weight:3});
    edges.push(Edge {start: 3, end: 4, weight:4});
    edges.push(Edge {start: 4, end: 5, weight:2});
    edges.push(Edge {start: 4, end: 6, weight:8});
    edges.push(Edge {start: 5, end: 6, weight:9});


    let inf = 10000;
    let mut k = 0;
    let mut flag = Vec::<bool>::new();
    let mut dist = Vec::<usize>::new();
    let mut path = Vec::<usize>::new();

    let mut matrix = Vec::<Vec<usize>>::new();
    for _ in 0..nodes_num {
        flag.push(false);
        dist.push(inf);
        path.push(nodes_num);

        let mut line = Vec::<usize>::new();
        for _ in 0..nodes_num {
            line.push(inf);
        }
        matrix.push(line);
    }

    for edge in edges {
        // assume that the weight is 1
        matrix[edge.start][edge.end] = edge.weight;
        matrix[edge.end][edge.start] = edge.weight;
    }

    //initialize
    for i in 0..nodes_num {
        matrix[i][i] = 0;
        dist[i] = matrix[src][i];
    }
    dist[src] = 0;

    flag[src] = true;

    for _i in 0..nodes_num {
        let mut min = inf;
        for j in 0..nodes_num {
            if flag[j] == false && dist[j] < min {
                min = dist[j];
                k = j;
            }
        }

        flag[k] = true;

        for j in 0..nodes_num {
            if flag[j] == false && dist[j] > min + matrix[k][j] {
                dist[j] = min + matrix[k][j];
                path[j] = k;
            }
        }
    }

    let mut pt = Vec::<usize>::new();
    let mut temp = dst;
    while temp != nodes_num {
        pt.push(temp);
        temp = path[temp];
    }
    pt.push(src);

    pt.reverse();

    println!("The shortest path from {} to {} is: {:?}",src, dst, pt);
}