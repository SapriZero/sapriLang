[struct.HolographicMemory]
derive: "Debug, Clone, Default"

[struct.HolographicMemory.fields]
entries: "VecDeque<MemoryEntry>"
max_size: "usize"

[struct.MemoryEntry]
derive: "Debug, Clone"

[struct.MemoryEntry.fields]
timestamp: "u64"
input: "String"
response: "String"

[impl.HolographicMemory]
functions: "new", "with_max_size", "remember", "recall", "recent", "save", "load", "len", "is_empty"

[impl.HolographicMemory.new]
params: ""
return_type: "Self"
is_static: "true"

[impl.HolographicMemory.with_max_size]
params: "max_size: usize"
return_type: "Self"
is_static: "true"

[impl.HolographicMemory.remember]
params: "&mut self, input: &str, response: &str"
return_type: "()"

[impl.HolographicMemory.recall]
params: "&self, input: &str"
return_type: "Option<String>"

[impl.HolographicMemory.recent]
params: "&self, n: usize"
return_type: "Vec<&MemoryEntry>"

[impl.HolographicMemory.save]
params: "&self, path: &str"
return_type: "Result<(), String>"

[impl.HolographicMemory.load]
params: "&mut self, path: &str"
return_type: "Result<(), String>"

[impl.HolographicMemory.len]
params: "&self"
return_type: "usize"

[impl.HolographicMemory.is_empty]
params: "&self"
return_type: "bool"
