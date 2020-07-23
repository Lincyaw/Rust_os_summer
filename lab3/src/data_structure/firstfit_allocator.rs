use super::Allocator;
use alloc::{vec, vec::Vec};


pub struct FirstFitAllocator {
    list: Vec<(usize,usize)>
}

impl Allocator for FirstFitAllocator {
    fn new(capacity: usize) -> Self {
        Self {
            list: vec![(0, capacity)],
        }
    }

    fn alloc(&mut self) -> Option<usize> {
        self.alloc_frames(1)
    }
    fn alloc_frames(&mut self,num:usize) -> Option<usize>
    {
        let size = self.list.len();
        for i in 0..size {
            let  (x,y) = self.list[i];
            if y-x<num {
                continue;
            }
            let re:usize = x;
            self.list[i] = ( x+num,y);
            if x== y {
                self.list.remove(i);
            }
            return Some(re)
        }
        return None;
    }
    fn dealloc(&mut self, index: usize,num:usize) {
        
        let size =self.list.len();
        if size ==0 {
            self.list.push((index, index + num));
        }
        if(size ==1){
            let (x,y) = self.list[0];
            if(x==index+num){
                self.list[0] = (index,y);
            }else if(y==index){
                self.list[0] = (x,y+num);
            }else if(x>index+num){
                self.list.insert(0,(index, index + num));
            }else{
                self.list.insert(1,(index, index + num));
            }
            return;
        }
        let (x,y)= self.list[0];
        if(x==index+num){
            self.list[0] = (index,y);
            return;
        }else if (x>index+num){
            self.list.insert(0,(index,index+num));
            return ;
        }
        
        for i in 0..size{
            let (x,y) = self.list[i];

            let (m,n) = self.list[i+1];
            if y<= index && m>=index+num{
                if(y==index && m == index+num){
                    self.list.remove(i+1);
                    self.list[i]=(x,n);
                    return 
                }
                if(y==index && m<index+num){
                    self.list[i] = (x,y+num);
                    return ;
                }
                if(y<index && m==index+num){
                    self.list[i+1] = (index,n);
                    return ;
                }
                if(y<index && m>index +num){
                    self.list.insert(i+1,(index,index+num));
                    return;
                }
            }else {continue;}
        }
        let (m,n)= self.list[size-1];
        if(n==index){
            self.list[size-1] = (m,n+num);
            return;
        }else if (n<index+num){
            self.list.insert(size,(index,index+num));
            return ;
        }
    }
}