#!/bin/bash

[ -d results ] || mkdir results/

WINRATE_TRIALS=10
WINRATE_DEPTH=3
LAST_DEPTH=5

echo ""
echo "==================== WINRATE ===================="

echo ""
echo "[ Minimax vs. Alpha-Beta vs. Alpha-Beta w/ Move Ordering ]"
echo ""
./apollo benchmark winrate mini:weight:$WINRATE_DEPTH ab:weight:$WINRATE_DEPTH ab-order:weight:$WINRATE_DEPTH -n $WINRATE_TRIALS > results/winrate-searches.csv

echo ""
echo "[ h-random vs. h-unit vs. h-weight vs. h-weight-mobility ]"
echo ""
./apollo benchmark winrate ab:random:$WINRATE_DEPTH ab:unit:$WINRATE_DEPTH ab:weight:$WINRATE_DEPTH ab:weight-mobility:$WINRATE_DEPTH -n $WINRATE_TRIALS > results/winrate-heuristics.csv

echo ""
echo "[ h-weight: depth 1 vs. depth 3 vs. depth $LAST_DEPTH ]"
echo ""
./apollo benchmark winrate ab:weight:1 ab:weight:3 ab:weight:$LAST_DEPTH -n $WINRATE_TRIALS > results/winrate-depth.csv
