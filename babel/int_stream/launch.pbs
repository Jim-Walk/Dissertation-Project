#!/bin/bash --login

# PBS job options (name, compute nodes, job time)
#PBS -N run_OMP_Stream
#PBS -l select=34
#PBS -l walltime=00:20:00
#PBS -A d167-s1893750
#PBS -l place=excl

# Change to the directory that the job was submitted from
# (remember this should be on the /work filesystem)
cd /lustre/home/d167/s1893750/Dissertation-Project/babel/int_stream

module load gcc

export RAYON_NUM_THREADS=1
echo "Running on 1 Thread"
./target/release/int_stream

export RAYON_NUM_THREADS=2
echo "Running on 2 Threads"
./target/release/int_stream

for ((i=4;i<=36;i+=4)); do 
   echo $i; 
   export RAYON_NUM_THREADS=$i
   echo "Running on" $i "Threads"
   ./target/release/int_stream
done
