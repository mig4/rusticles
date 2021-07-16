# text script experiment

I thought it would be fun to take a AWK/grep/column script I once constructed
to do some text manipulation and re-write it in [Rust][] 🦀😂

[The script](./the-script.sh) uses an [AWK script](./the-script.awk), `grep`
and `column` to process memory requests, limits and utilisation data in
[this file](./tests/resources/prometheus.resource-capacity.util.txt), summarise
them and output totals that make it easy to compare resource usage between
monolithic instances of Prometheus (rows marked _old_) and multiple shards they
were split into (rows marked _new_). The output is stored in
[old-new-comparison file](./tests/resources/prometheus.resource-capacity.old-new-comparison.txt).

For a bit of background, this was after a project to split a number of
monolithic [Prometheus][] instances into shards, initially one-per-cluster being
scraped. I wanted to compare and see how much more memory is used in total in
this setup.

The data was collected using a [fish-shell][] loop over [kubie][] executing a
kubectl [resource-capacity][] (plugin) command to collect resource usage data
from nodes and pods with label `app=prometheus`:

``` sh
for inst in $list_of_kube_contexts;
  echo "# $inst";
  kubie exec $inst monitoring \
    kubectl resource-capacity --pods --pod-labels=app=prometheus --util;
  or break;
end | tee prometheus.resource-capacity.util.txt
```

[fish-shell]: https://fishshell.com/
[kubie]: https://github.com/sbstp/kubie
[prometheus]: https://prometheus.io/
[resource-capacity]: https://github.com/robscott/kube-capacity
[rust]: https://www.rust-lang.org/
