import type { LatLng } from "../../../api/Api"

export class Coordinate2D {
  public readonly x: number
  public readonly y: number

  constructor(x: number, y: number) {
    this.x = x
    this.y = y
    Object.freeze(this)
  }

  static fromLatLng(latLng: LatLng): Coordinate2D {
    return new Coordinate2D(latLng.lng, latLng.lat)
  }

  normalizeBetween(min: Coordinate2D, max: Coordinate2D): Coordinate2D {
    return new Coordinate2D(
      Coordinate2D.interpolateAxis(min.x, max.x, this.x),
      Coordinate2D.interpolateAxis(min.y, max.y, this.y),
    )
  }

  invertYWithinUnitRange(): Coordinate2D {
    return new Coordinate2D(this.x, 1 - this.y)
  }

  toScreenPercent(): Coordinate2D {
    return new Coordinate2D(this.x * 100, this.y * 100)
  }

  private static interpolateAxis(
    min: number,
    max: number,
    source: number,
  ): number {
    const range = max - min
    if (range === 0) return 0.5

    return (source - min) / range
  }
}
