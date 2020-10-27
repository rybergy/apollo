#!/bin/bash

[ -d results ] || mkdir results/

PERFORMANCE_TRIALS=20
PERFORMANCE_DEPTH=6

echo ""
echo "==================== PERFORMANCE ===================="

echo ""
echo "[ Minimax vs. Alpha-Beta vs. Alpha-Beta w/ Move Ordering ]"
echo ""
./apollo benchmark performance mini:weight ab:weight ab-order:weight -n $PERFORMANCE_TRIALS -d $PERFORMANCE_DEPTH > results/performance-searches.csv

echo ""
echo "[ h-random vs. h-unit vs. h-weight vs. h-weight-mobility ]"
echo ""
./apollo benchmark performance ab-order:random ab-order:unit ab-order:weight ab-order:weight-mobility -n $PERFORMANCE_TRIALS -d $PERFORMANCE_DEPTH > results/performance-heuristics.csv
