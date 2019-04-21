struct Termino{
    states       : Vec<Vec<Vec<u8>>>,
    x            : isize,
    y            : isize,
    current_state: u8,
}

trait TerminoGenerator {
    fn new() -> Termino;
}

impl TerminoGenerator for TerminoI {
    fn new() -> Termino {
        Termino{
            states: vec![
                        vec![
                            vec![1, 1, 1, 1],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0],
                            vec![0, 0, 0, 0]
                            ],
                        vec![
                            vec![0, 1, 0, 0],
                            vec![0, 1, 0, 0],
                            vec![0, 1, 0, 0],
                            vec![0, 1, 0, 0]
                            ]
                        ],
            x            : 4,
            y            : 0,
            current_state: 0,
        }
    }
}

impl TerminoGenerator for TerminanoJ {
    fn new () -> Termino {
        states: vec![
                    vec![
                        vec![2, 2, 2, 0],
                        vec![2, 0, 0, 0],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0]
                    ],
                    vec![
                        vec![2, 2, 0, 0],
                        
                    ]
        ]
    }
}