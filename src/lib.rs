mod abc380;
mod abc381;
// mod daily_training;

#[cfg(test)]
mod abc381_a {
    use super::*;
    #[test]
    fn test1() {
        let result= abc381::q1_a(5, "11/22".to_string());
        assert_eq!(result, "Yes".to_string());
    }
    #[test]
    fn test2() {
        let result= abc381::q1_a(1, "/".to_string());
        assert_eq!(result, "Yes".to_string());
    }
    #[test]
    fn test3() {
        let result= abc381::q1_a(4, "1/22".to_string());
        assert_eq!(result, "No".to_string());
    }
    #[test]
    fn test4() {
        let result= abc381::q1_a(5, "22/11".to_string());
        assert_eq!(result, "No".to_string());
    }
}

// #[cfg(test)]
// mod abc380_a {
//     use super::*;

//     #[test]
//     fn test1() {
//         let result= abc380::q1_a(123233);
//         assert_eq!(result, "Yes".to_string());
//     }
//     #[test]
//     fn test2() {
//         let result= abc380::q1_a(323132);
//         assert_eq!(result, "Yes".to_string());
//     }
//     #[test]
//     fn test3() {
//         let result= abc380::q1_a(123234);
//         assert_eq!(result, "No".to_string());
//     }
//     #[test]
//     fn test4() {
//         let result= abc380::q1_a(500000);
//         assert_eq!(result, "No".to_string());
//     }

// }

// #[cfg(test)]
// mod q_2024_11_19_1530 {
//     use super::*;

//     #[test]
//     fn a1() {
//         let result= daily_training::ez_a(Vec::from([4,3,3,6,2,5,10]));
//         assert_eq!(result, "Takahashi".to_string());
//     }
//     #[test]
//     fn a2() {
//         let result= daily_training::ez_a(Vec::from([3,1,4,1,5,9,2]));
//         assert_eq!(result, "Aoki".to_string());
//     }
//     #[test]
//     fn a3() {
//         let result= daily_training::ez_a(Vec::from([1,1,1,1,1,1,1]));
//         assert_eq!(result, "Draw".to_string());
//     }
// }