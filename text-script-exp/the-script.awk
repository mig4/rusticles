BEGIN{
  # we print totals when encountering a new section, initialise with a dummy
  # installation so this works in the first section; we'll clean-up after
  inst="null";
  reset_totals();
  print "INSTALLATION PROM SHARDS REQUESTS LIMITS UTIL";
};

# reset counters
function reset_totals() {
  inst_sh_count=0;
  inst_sh_req_total=0;
  inst_sh_lim_total=0;
  inst_sh_util_total=0;
};

# print totals for current section/installation based on the running counters
function print_inst_new_totals() {
  printf "%s new %d %dMi %dMi %dMi\n",
    inst, inst_sh_count, inst_sh_req_total, inst_sh_lim_total, inst_sh_util_total;
};

# new section/installation
/^# / {
  # print out what we've accumulated for the previous section and prepare for
  # collecting data from the new section
  print_inst_new_totals();
  reset_totals();
  inst=$2;
  next
};

# Prometheus running in the "monitoring" namespace, that's the old monolithic
# instance; print out a row with fields 10 (memory requests), 12 (memory
# limits) and 14 (memory utilisation)
$2 == "monitoring" {
  print inst, "old", 1, $10, $12, $14
};

# Prometheus running in a per-shard namespace, that's the new one; collect data
# in running counters; Note that AWK ignores any non-numeric suffixes.
$2 ~ /-prometheus$/ {
  inst_sh_count+=1;
  inst_sh_req_total+=$10;
  inst_sh_lim_total+=$12;
  inst_sh_util_total+=$14;
};

# EOF, print totals for the last section/installation.
END{
  print_inst_new_totals();
};
