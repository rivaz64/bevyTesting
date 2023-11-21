include!("collicions.rs");
use std::mem::MaybeUninit;
#[derive(Clone)]
struct Cell<const N: usize>where
{
    count: usize,
    data : [*mut Transform; N],
}

#[derive(Component)]
struct Grid2d<const N:usize>{
    extent_x : usize,
    extent_y : usize,
    cells : Vec<Cell<N>>,
    collicion: CollicionDynamicCallback,
    callback : CallbackDynamic
}

impl<const N:usize> Cell<N> where
{
    fn new() -> Self {
        Cell{count:0,data:unsafe { MaybeUninit::uninit().assume_init() }}
    }
    fn insert(&mut self, item:*mut Transform){
        if self.count<N{
            self.data[self.count]=item;
            self.count+=1;
        }
    }
}

impl<const N:usize> Grid2d<N>
{
    fn new(rows: usize, cols: usize, collicion :CollicionDynamicCallback, callback: CallbackDynamic) -> Self {
        //let data = vec![; rows* cols];
        Grid2d { 
            extent_x:rows,
            extent_y: cols,
            cells: vec![Cell::<N>::new();rows*cols], 
            collicion:collicion,
            callback:callback
        }
    }

    fn clear(&mut self){
        for cell in self.cells.iter_mut(){
            cell.count = 0;
        }
    }

    unsafe fn insert(&mut self, x:usize, y:usize,item:&mut Transform){
        if x>=0 && x<self.extent_x && y>=0 && y<self.extent_y{
            self.compare(item,y*self.extent_x+x);
            self.compare(item,y*self.extent_x+x+1);
            self.compare(item,y*self.extent_x+x-1);
            self.compare(item,(y+1)*self.extent_x+x);
            self.compare(item,(y-1)*self.extent_x+x);
            self.compare(item,(y+1)*self.extent_x+x+1);
            self.compare(item,(y-1)*self.extent_x+x+1);
            self.compare(item,(y+1)*self.extent_x+x-1);
            self.compare(item,(y-1)*self.extent_x+x-1);
            self.cells[y*self.extent_x+x].insert(item);
        }
    }
    
    unsafe fn compare(&self, item:&mut Transform,cell_id: usize){
        let cell = &self.cells[cell_id];
        for i in 0..cell.count{
            //self.collicion();
            (self.collicion)(item,&mut*cell.data[i],self.callback);
        }
    }
}

