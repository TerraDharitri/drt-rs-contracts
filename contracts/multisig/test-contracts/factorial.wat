(module
  (type (;0;) (func (param i32 i64)))
  (type (;1;) (func (param i32 i32)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func))
  (type (;4;) (func (param i32) (result i32)))
  (type (;5;) (func (param i32 i32) (result i32)))
  (type (;6;) (func (param i32 i32 i32)))
  (type (;7;) (func (param i32)))
  (import "env" "bigIntSetInt64" (func (;0;) (type 0)))
  (import "env" "bigIntGetUnsignedArgument" (func (;1;) (type 1)))
  (import "env" "getNumArguments" (func (;2;) (type 2)))
  (import "env" "signalError" (func (;3;) (type 1)))
  (import "env" "checkNoPayment" (func (;4;) (type 3)))
  (import "env" "bigIntSign" (func (;5;) (type 4)))
  (import "env" "bigIntCmp" (func (;6;) (type 5)))
  (import "env" "bigIntMul" (func (;7;) (type 6)))
  (import "env" "bigIntAdd" (func (;8;) (type 6)))
  (import "env" "bigIntFinishUnsigned" (func (;9;) (type 7)))
  (func (;10;) (type 2) (result i32)
    (local i32)
    call 11
    local.tee 0
    i64.const 1
    call 0
    local.get 0)
  (func (;11;) (type 2) (result i32)
    (local i32)
    i32.const 0
    i32.const 0
    i32.load offset=131100
    i32.const -1
    i32.add
    local.tee 0
    i32.store offset=131100
    local.get 0)
  (func (;12;) (type 2) (result i32)
    (local i32)
    i32.const 0
    call 11
    local.tee 0
    call 1
    local.get 0)
  (func (;13;) (type 7) (param i32)
    block  ;; label = @1
      call 2
      local.get 0
      i32.ne
      br_if 0 (;@1;)
      return
    end
    i32.const 131072
    i32.const 25
    call 3
    unreachable)
  (func (;14;) (type 3)
    call 4
    i32.const 0
    call 13)
  (func (;15;) (type 3)
    (local i32 i32 i32 i32)
    call 4
    i32.const 1
    call 13
    call 12
    local.set 0
    call 10
    local.set 1
    block  ;; label = @1
      local.get 0
      call 5
      i32.eqz
      br_if 0 (;@1;)
      call 10
      local.set 2
      call 10
      local.set 3
      loop  ;; label = @2
        block  ;; label = @3
          local.get 3
          local.get 0
          call 6
          i32.const 0
          i32.le_s
          br_if 0 (;@3;)
          local.get 2
          local.set 1
          br 2 (;@1;)
        end
        local.get 2
        local.get 2
        local.get 3
        call 7
        local.get 3
        local.get 3
        local.get 1
        call 8
        br 0 (;@2;)
      end
    end
    local.get 1
    call 9)
  (func (;16;) (type 3))
  (table (;0;) 1 1 funcref)
  (memory (;0;) 3)
  (global (;0;) (mut i32) (i32.const 131072))
  (global (;1;) i32 (i32.const 131104))
  (global (;2;) i32 (i32.const 131104))
  (export "memory" (memory 0))
  (export "init" (func 14))
  (export "factorial" (func 15))
  (export "callBack" (func 16))
  (export "upgrade" (func 14))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data (;0;) (i32.const 131072) "wrong number of arguments")
  (data (;1;) (i32.const 131100) "\9c\ff\ff\ff"))
