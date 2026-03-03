# Toy Async Executor

This project is a toy async executor built in rust, it uses another thread to simulate the OS, It uses channels to set a Task sleep and wake it up after a period of time.

Currently the Tasks doesn't hold state, When they are awaken then start from the beginning again.

_This project is built as a demonstration to explain concepts not to be used in production_
