package aoc;

import com.google.common.base.Preconditions;
import com.google.common.collect.Lists;
import lombok.Value;

import java.util.List;

/**
 * Hex grid
 *
 *  y __ x
 *   /  \
 *   \__/
 *     z
 *
 *
 * https://www.redblobgames.com/grids/hexagons/
 */
@Value
public class HexPoint {
    public static HexPoint ZERO = new HexPoint(0,0, 0);
    public static HexPoint NW = new HexPoint(-1,1, 0);
    public static HexPoint N = new HexPoint(0,1, -1);
    public static HexPoint NE = new HexPoint(1,0, -1);

    public static HexPoint SW = new HexPoint(-1,0, 1);
    public static HexPoint S = new HexPoint(0,-1, 1);
    public static HexPoint SE = new HexPoint(1,-1, 0);

    int x, y, z;

    public HexPoint(int x, int y, int z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public HexPoint add(HexPoint o) {
        return new HexPoint(
                this.x + o.x,
                this.y + o.y,
                this.z + o.z
        );
    }

    public double manhattanDistance(HexPoint t) {
        return (Math.abs(this.getX() - t.getX()) +
                Math.abs(this.getY() - t.getY()) +
                Math.abs(this.getZ() - t.getZ())) / 2;
    }

    public List<HexPoint> neighbors() {
        return Lists.newArrayList(
                this.add(N),
                this.add(S),
                this.add(NE),
                this.add(NW),
                this.add(SE),
                this.add(SW)
        );
    }
}
