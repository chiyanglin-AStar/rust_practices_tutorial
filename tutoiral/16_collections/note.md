# Rust Collections 
## vector 
Sr.No		Method				Signature & Description
1.new()		pub fn new()->Vect		Constructs a new, empty Vec. The vector will not allocate until elements are pushed onto it.

2.push()	pub fn push(&mut self, value: T) Appends an element to the back of a collection.

3.remove()	pub fn remove(&mut self, index: usize) -> T    Removes and returns the element at position index within the vector, shifting all elements after it to the left.

4.contains()	pub fn contains(&self, x: &T) -> bool   Returns true if the slice contains an element with the given value.

5.len()	        pub fn len(&self) -> usize     		Returns the number of elements in the vector, also referred to as its 'length'.

##  Syntax: Creating a HashMap

let mut instance_name = HashMap::new();

Sr.No		Method						Signature & Description
1.insert()	pub fn insert(&mut self, k: K, v: V) -> Option  Inserts a key/value pair, if no key then None is returned. After update, old value is returned.

2.len()		pub fn len(&self) -> usize			Returns the number of elements in the map.

3.get()		pub fn get<Q: ?Sized>(&lself, k: &Q) -> Option<&V> where K:Borrow Q:Hash+ Eq

Returns a reference to the value corresponding to the key.

4.iter()  	pub fn iter(&self) -> Iter<K, V>		An iterator visiting all key-value pairs in arbitrary order. The iterator element type is (&'a K, &'a V).

5.contains_key	pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool 	Returns true if the map contains a value for the specified key.

6.remove()	pub fn remove_entry<Q: ?Sized>(&mut self, k: &Q) -> Option<(K, V)>

Removes a key from the map, returning the stored key and value if the key was previously in the map.

## Syntax: Creating a HashSet
let mut hash_set_name = HashSet::new();

Sr.No	Method	Signature & Description
1	insert()	
pub fn insert(&mut self, value: T) -> bool

Adds a value to the set. If the set did not have this value present, true is returned else false.

2	len()	
pub fn len(&self) -> usize

Returns the number of elements in the set.

3	get()	
pub fn get<Q:?Sized>(&self, value: &Q) -> Option<&T> where T: Borrow,Q: Hash + Eq,

Returns a reference to the value in the set, if any that is equal to the given value.

4	iter()	
pub fn iter(&self) -> Iter

Returns an iterator visiting all elements in arbitrary order. The iterator element type is &'a T.

5	contains_key	
pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool

Returns true if the set contains a value.

6	remove()	
pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool

Removes a value from the set. Returns true if the value was present in the set.
