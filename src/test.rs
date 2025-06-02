const INF: i32 = i32::MAX / 2;

pub fn get_test_matrices() -> Vec<Vec<Vec<i32>>> {
    vec![
        vec![
            vec![0,  12, 10, INF, INF, INF, 12], 
            vec![12, 0,  12, 10,  INF, INF, INF],
            vec![10, 12, 0,  8,   12,  INF, INF],
            vec![INF,10, 8,  0,   11,  3,   9 ],
            vec![INF,INF,12, 11,  0,   10,  11],
            vec![INF,INF,INF,3,   10,  0,   6 ], 
            vec![12, INF,INF,9,   11,  6,   0 ],
        ],
        vec![
            vec![0, 10, 15],
            vec![10, 0, 20],
            vec![15, 20, 0],
        ],
        vec![
            vec![0, 20, 42, 25],
            vec![20, 0, 30, 34],
            vec![42, 30, 0, 10],
            vec![25, 34, 10, 0],
        ],
        vec![
            vec![0, 10, INF, 20, 15],
            vec![10, 0, 25, INF, 30],
            vec![INF, 25, 0, 5, 12],
            vec![20, INF, 5, 0, 8],
            vec![15, 30, 12, 8, 0],
        ],
        vec![
            vec![0, 5, 10, 5],
            vec![5, 0, 5, 10],
            vec![10, 5, 0, 5],
            vec![5, 10, 5, 0],
        ],
        vec![
            vec![0, 15, INF, INF, INF, 20],
            vec![15, 0, 18, INF, INF, INF],
            vec![INF, 18, 0, 12, INF, INF],
            vec![INF, INF, 12, 0, 14, INF],
            vec![INF, INF, INF, 14, 0, 16],
            vec![20, INF, INF, INF, 16, 0],
        ],
        vec![
            vec![0, 1, 100],
            vec![1, 0, 1],
            vec![100, 1, 0],
        ],
        vec![
            vec![0, 3, 3, 3, 3],
            vec![3, 0, INF, INF, INF],
            vec![3, INF, 0, INF, INF],
            vec![3, INF, INF, 0, INF],
            vec![3, INF, INF, INF, 0],
        ],
    ]
}