#!/bin/bash --login

# PBS job options (name, compute nodes, job time)
#PBS -N k_means_rust
#PBS -l select=36
#PBS -l walltime=00:15:00
#PBS -A d167-s1893750
#PBS -l place=excl

# Change to the directory that the job was submitted from
# (remember this should be on the /work filesystem)
cd /lustre/home/d167/s1893750/Dissertation-Project/k_means/k_means

module load gcc
module load intel-compilers-18
module load netcdf-parallel/4.5.0-gcc6-mpt214
module load hdf5-1.10.1-gcc-6.3.0-sil5apz

export RAYON_NUM_THREADS=1
echo "Running on 1 Thread"
./target/release/k_means

 export RAYON_NUM_THREADS=2
 echo "Running on 2 Threads"
./target/release/k_means
 
 for ((i=4;i<=36;i+=4)); do 
    echo $i; 
    export RAYON_NUM_THREADS=$i
    echo "Running on" $i "Threads"
    ./target/release/k_means
 done
