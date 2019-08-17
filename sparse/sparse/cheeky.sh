#!/usr/bin/bash

# PBS job options (name, compute nodes, job time)
#PBS -N sparse_100_iter_para_init
#PBS -l select=36
#PBS -l walltime=00:30:00
#PBS -A d167-s1893750
#PBS -l place=excl

# Change to the directory that the job was submitted from
# (remember this should be on the /work filesystem)

module load gcc
module load intel-compilers-18
module load intel-tools-18/18.0.5.274

echo "Running on 1 Thread"
perf stat -e r5301c7 -e r5302c7 -e r5304c7 -e r5308c7 -e r5310c7 -e r5320c7 ./target/release/sparse -g 12 -s 15 -n 101 -t 1 > 1.dat

export RAYON_NUM_THREADS=2
echo "Running on 2 Threads"
perf stat -e r5301c7 -e r5302c7 -e r5304c7 -e r5308c7 -e r5310c7 -e r5320c7 ./target/release/sparse -g 12 -s 15 -n 101 -t 2 > 2.dat
 
 for ((i=4;i<=36;i+=4)); do 
    echo "Running on" $i "Threads"
    d="$i.dat"
   perf stat -e r5301c7 -e r5302c7 -e r5304c7 -e r5308c7 -e r5310c7 -e r5320c7 ./target/release/sparse -g 12 -s 15 -n 101 -t $i > $d
 done
