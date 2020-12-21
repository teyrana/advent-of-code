

static TEST_DATA: [i32;6] = [1721,979,366,299,675,1456];

static LIVE_DATA: [i32;200] = [1509,1857,1736,1815,1576,1970,1567,1778,1508,1833,1377,1890,1375,1396,1102,1639,1818,1469,1138,1333,1906,1557,1686,
        1712,1990,1930,1761,1881,1551,1627,1801,1728,1960,1407,1832,1842,1393,1870,1295,1528,251,1945,1589,1850,1650,1793,1997,1758,1477,1697,1081,
        1825,1899,1171,1104,1839,1974,1630,1831,1671,1723,1811,1489,1647,1486,1107,1786,1680,1942,1640,1112,1703,1315,1769,1966,997,2010,1635,1196,
        383,1986,1860,1743,1756,1555,1111,1823,48,1953,1083,1804,1933,1626,1895,1807,1669,1783,389,1821,1883,1114,1587,1941,1725,1646,456,1550,1939,
        1975,1324,1201,1018,1001,1402,1885,1481,1633,1781,1622,1822,1559,1696,1510,1251,1732,1790,1813,1695,1121,704,1964,1984,1763,1656,1183,1771,1276,
        1764,1810,1992,1213,1840,1318,1965,1943,1549,1768,1506,1949,1739,1852,1787,1570,1988,1357,1909,1837,561,1994,1777,1547,1925,1897,1817,1677,
        1668,1982,1667,1753,1041,1826,1961,1797,1765,1720,1835,1688,1705,1744,1977,1971,1775,1782,1661,1385,1162,1755,1846,1674,1698,1882,1766,1820,
        1531,1577,1710,1382,1246,1864,1702];


// fn build_heap(){
//     let heap = 0;

//     return heap;
// }



fn main() {
    
    println!(">> selecting input");
    let mut input = LIVE_DATA.clone();
    
    let target = 2020;

    println!(">> sorting...");
    input.sort();
    
    // let mut lcuri: usize = 0; // left cursor index -- points to the smaller number
    // let mut rcuri: usize = input.len()-1;  // right cursor index points to the larger number
    // println!(">> Set initial cursors guess: [{},{}]", lcuri, rcuri );
    // while (sum != 2020) &&  (lcuri < rcuri) { 
    //     let sum = input[lcuri] + input[rcuri];
    // 
    //     if sum < target {
    //         lcuri += 1;
    //     }else if sum > target {
    //         rcuri -= 1;
    //     }else{
    //         break;
    //     }
    // }
    // 
    // println!("<< Step 1: [{},{}] => ({}, {}) => {}", lcuri, rcuri, input[lcuri], input[rcuri], input[lcuri]+input[rcuri] );
    // println!("    ==>> the product is: {} ", input[lcuri] * input[rcuri] );
    // 
    // return;
    
    // let mut i: usize = 0;
    // let mut j: usize = 0;
    // let mut k: usize = 0;
    println!(">> Starting Brute Search, from {} to {} ...", 0, input.len()-1);
    for i in 0..input.len()-1 {
        for j in 0..input.len()-1 {
            for k in 0..input.len()-1 {
                if (i==j) || (j==k) {
                    continue; // no duplicate numbers!
                } else if ( i > j) || (j>k) {
                    continue; // this reduces the seach space, by eliminating duplicates
                }

                let sum = input[i] + input[j] + input[k];
                // println!("    ::iteration: @({}, {}, {}) => ({}, {}, {}) => {}", i,j,k, input[i], input[j], input[k], sum );

                if sum == target {
                    println!("<< found: @({}, {}, {}) => ({}, {}, {}) => {}", i,j,k, input[i], input[j], input[k], sum );
                    println!("    ==>> the product is: {} ", input[i]*input[j]*input[k] );
                }
            }
        }
    }

    // println!("<< final: @({}, {}, {}) => ({}, {}, {}) => {}", i,j,k, input[i], input[j], input[k], sum );
    // println!("    ==>> the product is: {} ", input[i]*input[j]*input[k] );
    println!(" ==== END ==== ");
}
