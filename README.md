# procos

![virtual memory model](./virtual_memory_model.png)

![task model](./task_model.png)


## Trap flow

### syscall

__alltraps -> trap_handler -> trap_return -> __restore

![trap flow](./trap.png)

### run app at the first time

__restore

## Trap context vs Task context

- Trap context
  - different modes(user vs kernel/supervisor) --> handled by CPU automatically
  - different stacks(user vs kernel of the same app)
  - different memory spaces(user vs kernel)

- Task context
  - only stack change(different kernel stacks of apps) --> almost similiar to normal function call
