(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (result i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32 i32 i32) (result i32)))
  (type (;4;) (func))
  (type (;5;) (func (param i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32) (result i32)))
  (import "env" "bigIntGetUnsignedArgument" (func (;0;) (type 0)))
  (import "env" "getNumArguments" (func (;1;) (type 1)))
  (import "env" "signalError" (func (;2;) (type 0)))
  (import "env" "mBufferStorageLoad" (func (;3;) (type 2)))
  (import "env" "mBufferToBigIntUnsigned" (func (;4;) (type 2)))
  (import "env" "mBufferFromBigIntUnsigned" (func (;5;) (type 2)))
  (import "env" "mBufferStorageStore" (func (;6;) (type 2)))
  (import "env" "mBufferSetBytes" (func (;7;) (type 3)))
  (import "env" "checkNoPayment" (func (;8;) (type 4)))
  (import "env" "bigIntFinishUnsigned" (func (;9;) (type 5)))
  (import "env" "bigIntAdd" (func (;10;) (type 6)))
  (func (;11;) (type 1) (result i32)
    (local i32)
    i32.const 0
    call 12
    local.tee 0
    call 0
    local.get 0)
  (func (;12;) (type 1) (result i32)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=131100
    i32.const -1
    i32.add
    local.tee 0
    i32.store offset=131100
    local.get 0)
  (func (;13;) (type 5) (param i32)
    block  ;; label = @1
      call 1
      local.get 0
      i32.ne
      br_if 0 (;@1;)
      return
    end
    i32.const 131072
    i32.const 25
    call 2
    unreachable)
  (func (;14;) (type 7) (param i32) (result i32)
    (local i32)
    local.get 0
    call 12
    local.tee 1
    call 3
    drop
    local.get 1
    call 12
    local.tee 0
    call 4
    drop
    local.get 0)
  (func (;15;) (type 0) (param i32 i32)
    (local i32)
    call 12
    local.tee 2
    local.get 1
    call 5
    drop
    local.get 0
    local.get 2
    call 6
    drop)
  (func (;16;) (type 1) (result i32)
    (local i32)
    call 12
    local.tee 0
    i32.const 131097
    i32.const 3
    call 7
    drop
    local.get 0)
  (func (;17;) (type 4)
    (local i32)
    call 8
    i32.const 1
    call 13
    call 11
    local.set 0
    call 16
    local.get 0
    call 15)
  (func (;18;) (type 4)
    call 8
    i32.const 0
    call 13
    call 16
    call 14
    call 9)
  (func (;19;) (type 4)
    (local i32 i32 i32)
    i32.const 1
    call 13
    call 11
    local.set 0
    call 16
    local.tee 1
    call 14
    local.tee 2
    local.get 2
    local.get 0
    call 10
    local.get 1
    local.get 2
    call 15)
  (func (;20;) (type 4))
  (table (;0;) 1 1 funcref)
  (memory (;0;) 3)
  (global (;0;) (mut i32) (i32.const 131072))
  (global (;1;) i32 (i32.const 131104))
  (global (;2;) i32 (i32.const 131104))
  (export "memory" (memory 0))
  (export "init" (func 17))
  (export "getSum" (func 18))
  (export "add" (func 19))
  (export "callBack" (func 20))
  (export "upgrade" (func 17))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data (;0;) (i32.const 131072) "wrong number of argumentssum")
  (data (;1;) (i32.const 131100) "\9c\ff\ff\ff"))
