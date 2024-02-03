pub mod search {
    pub fn linear_search(arr : &[isize], key : isize) -> Option<isize> {
        for i in 0..arr.len() {
            if arr[i] == key {
                return  Some(i as isize);
            }
        }

        None
    }

    pub fn binary_search(arr : &[isize], key : isize) -> Option<isize> {
        let mut l = 0;
        let mut h = arr.len() -1 ;

        let mut mid =( l + h )/2;

        while l <= h {
            if(key ==arr[mid]) {
                return  Some(mid as isize);
            } else if( key < arr[mid]) {
                h = mid -1;
            } else {
                l = mid +1;
            }
        }

        None

    }

    pub fn binary_search_rec(arr: &[isize], key : isize) -> Option<isize> {
       fn helper(arr: &[isize],l : isize, h : isize, key : isize) -> Option<isize> {
           if l > h {
             None
           }
           else {
                let mid = (l + h) /2;

               if(arr[mid as usize ] == key ) {
                   return Some(mid);
               } else if (arr[mid as usize] < key) {
                   helper(arr,mid+1,h, key)
               } else {
                   helper(arr,l , mid- 1 ,key)
               }
           }
       }
        helper(arr, 0, (arr.len() -1) as isize , key )
    }
}