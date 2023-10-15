# filters_and_sketches
Comparing the performance of several frequency sketches and counting filters and similar algorithms in terms of memory, error, and throughput
Currently implemented sketches and filters include a plain hashtable, a counting cuckoo filter, count-min sketch (CMS), and Space Saving.
The first three can be invoked (as an option) with the Nitro optimization of [b], while the latter can be invoked (as an option) with the RAP optimization of [c]
NitroCMS can be also invoked with a compact optimization, where the allocated size of the table is reduced by the sampling rate.
The default implementation of CMS implements the minimal increment optimization; this can be turned off.
The results of some experiments are summaized in paper [a].

References:
+ [a] Roy Friedman. "An Evaluation of Software Sketches". https://arxiv.org/abs/2309.03045.
+ [b] Zaoxing Liu, Ran Ben-Basat, Gil Einziger, Yaron Kassner, Vladimir Braverman, Roy Friedman, and Vyas Sekar. "Nitrosketch: Robust and General Sketch-based Monitoring in Software Switches". https://dl.acm.org/doi/10.1145/3341302.3342076.
+ [c] Ran Ben Basat, Xiaoqi Chen, Gil Einziger, Roy Friedman and Yaron Kassner. "Randomized Admission Policy for Efficient Top-k, Frequency, and Volume Estimation". https://ieeexplore.ieee.org/document/8734012

The runtime options include:
+  --file-path: The location of the trace/workload
+  --ds-type: The sketch/filter to be used. Permitted valued include HASH, NitroHash, CMS, NitroCMS, SpaceSaving, Cuckoo, NitroCuckoo
+  --time-type: In cae of timing measurements, which test to run: READTIME (prefill the table with the trace, then time reading all items according to the trace), WRITETIME (time inserting all items according to the trace), RWTIME (time inserting all items where immediately after each insert perform a read as well)
+  --error: The theoretical error guarantee parameter epsilon, treated according to the sketch/filter type chosen, default 0.01
+  --confidence: The probability delta of meating the theoretical error guarantee, treated according to the sketch/filter type chosen, default 0.01
+  --max-size: Unused at the moment - reserved for a future fingerprint based implementation
+  --fp_size: Unused at the moment - reserved for a future fingerprint based implementation
+  --sample: Sampling probability for the Nitro optimization
+  --avoid-mi: Do not perform the minimal increment (conservative update) optimization for CMS
+  --rap: Implement the RAP optimization in case of SpaceSaving
+  --compare: Boolean parameter; if set, compare accuracy and memory usage instead of timing information
+  --compact:  Allocate space only for a fraction of the workload according to the sampling parameter in case of NitroCuckoo
+  --verbose: Print extra debug info to the standard output
  
  The format of each line of a trace file is expacted to be <src_ip_1> <src_ip_2> <src_ip_3> <src_ip_4> <dst_ip_1> <dst_ip_2> <dst_ip_3> <dst_ip_4> [<something>], where each src_ip_i and dst_ip_i are a single byte (0-255). Inconsistencies are defaulted to 0.
  [TODO: document output format]

  The .bat files include exampels on how to use the file that were used in the paper summarizing the results.

  The directory python includes an example python code that generates comparisson graphs from all outfiles located in a given directory.
  The parameters include:
  +  --path: Directory where the files are, default='..\\..\\rust-projects\\filters_and_sketches\\results'
  +  --restrict: Influences which sketches and filters will be used for the graphs, defaults to BASIC:
  +    BASIC = ['SpaceSaving', 'SpaceSaving-RAP', 'CMS', 'NitroCMS', 'HASH', 'NitroHash', 'Cuckoo', 'NitroCuckoo']
  +    OPTS-FULL = ['CMS', 'NitroCMS', 'CMS-NOMI', 'Cuckoo', 'NitroCuckoo', 'NitroCuckoo-SMALL']
  +    OPTS = ['CMS', 'CMS-NOMI', 'NitroCuckoo', 'NitroCuckoo-SMALL']
  +    NOMI = ['CMS', 'CMS-NOMI']
  +    NITRO = ['Cuckoo', 'NitroCuckoo', 'NitroCuckoo-SMALL']
  +    bad values results in all sketches and filters
  
