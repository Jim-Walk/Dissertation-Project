#!/bin/bash --login

# PBS job options (name, compute nodes, job time)
#PBS -N 1GB_Rust_chunk_100MB
#PBS -l select=34
#PBS -l walltime=00:15:00
#PBS -A d167-s1893750
#PBS -l place=excl

# Change to the directory that the job was submitted from
# (remember this should be on the /work filesystem)
cd /lustre/home/d167/s1893750/Dissertation-Project/babel/babel_stream

module load gcc
module load intel-compilers-18
module load intel-tools-18/18.0.5.274
module load perfcatcher

export RAYON_NUM_THREADS=1
echo "Running on 1 Thread"
./target/release/babel_stream -s 125000000

 export RAYON_NUM_THREADS=2
 echo "Running on 2 Threads"
 ./target/release/babel_stream -s 125000000
 
 for ((i=4;i<=36;i+=4)); do 
    echo $i; 
    export RAYON_NUM_THREADS=$i
    echo "Running on" $i "Threads"
    ./target/release/babel_stream -s 125000000
 done
