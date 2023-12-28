# STEP 0: Make sure there is no left-over profiling data from previous runs
rm -rf /tmp/pgo-data

# STEP 1: Build the instrumented binaries
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" \
    rustup run nightly cargo build --release --target=x86_64-unknown-linux-gnu

# STEP 2: Run the instrumented binaries with some typical data
for i in {1..3}; do
    ./target/x86_64-unknown-linux-gnu/release/cell_sim & pid=$!
    sleep 60
    kill -SIGINT $pid
done

# STEP 3: Merge the `.profraw` files into a `.profdata` file
llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data

# STEP 4: Use the `.profdata` file for guiding optimizations
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" \
    rustup run nightly cargo build --release --target=x86_64-unknown-linux-gnu

#upx -9 ./target/x86_64-unknown-linux-gnu/release/cell_sim -o ./target/x86_64-unknown-linux-gnu/release/cell_sim_upx
