LDFLAGS = -L. target/release/libidentity_c.a -lpthread -ldl

OBJS = main.o

main: $(OBJS)
	cargo build --release
	$(CC) -o main $^ $(LDFLAGS)

clean:
	$(RM) *~ *.o main