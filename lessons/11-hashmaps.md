## Hash maps

- HashMap<K, V> stores a mapping of type K to values of type V using a hashing function
- hash maps are useful when you don't want to look up data based on index
- To use hash maps, we say `use std::collections::HashMap`
- Hashmap is not used by default - needs to specifically imported into the current context -> this is because hashmap is lesser used in comparison to vectors and strigs

- unlike vectors, there is no built in macro to initialize a hash map

- to insert a specific hash map, we do

```
   let mut scores = HashMap::new();

   scores.insert(String::from("blue"), 34);
```


- to get a specific value against a key -> note that this is of Option<&V> type. 

```
   let color = String::from("blue");

   let opt_val = scores.get(&color).copied(); // by value instead of reference
   
   let val = opt_val.unwrap_or(0); // this gives value if some or 0 if none


```


- We can loop over all keys and values by following

```
   for (key, value) in &scores{

      println!("{key} : {value}");
   }
```


- Ownership - for types that implement `Copy` trait like u32, values will be copied into hashmap. for types that dont, ie. String, the ownership will be transferred to the hash map

- So in the below case, we can't use city after it has been moved into hashmap

```

  fn city_database(){
    

     let city = String::from("nyc");

     let zip  = 19425;

     let mut zipcodes = HashMap::new();
     zipcodes.insert(city, zip);

    // we cannot use city anymore as it is moved
   // we can however use zip
  }
```

- if we use references, values will not be moved into the hashmap. And the reference should be valid as long as hash map itself is valid

- Each key can hold only one value and keys are unique in hash map

- hash maps have a method called `entry` that returns Entry Enum -> Entry has a method or_insert -> if not exists, this will insert with a specific value. A very useful method because it inserts value and returns a mutable reference to that new value.

- technique is much cleaner than writing logic ourselves

- 

