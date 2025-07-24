// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import type { Option } from '@tb-dev/utils';

export class CoordImpl implements Coord {
  public readonly x: number;
  public readonly y: number;

  #id: Option<string>;
  #xOutside: Option<boolean>;
  #yOutside: Option<boolean>;

  private constructor(coord: Coord) {
    this.x = coord.x;
    this.y = coord.y;
  }

  public is(other: Coord) {
    return this.x === other.x && this.y === other.y;
  }

  public isOutside() {
    return this.isXOutside() || this.isYOutside();
  }

  public isXOutside() {
    if (typeof this.#xOutside !== 'boolean') {
      const size = NIL.world.refs().continentSize.value;
      this.#xOutside = this.x < 0 || this.x >= size;
    }

    return this.#xOutside;
  }

  public isYOutside() {
    if (typeof this.#yOutside !== 'boolean') {
      const size = NIL.world.refs().continentSize.value;
      this.#yOutside = this.y < 0 || this.y >= size;
    }

    return this.#yOutside;
  }

  public format() {
    this.#id ??= `${this.formatX()}|${this.formatY()}`;
    return this.#id;
  }

  public formatX() {
    return CoordImpl.intl.format(this.x);
  }

  public formatY() {
    return CoordImpl.intl.format(this.y);
  }

  public toIndex() {
    return CoordImpl.toIndex(this);
  }

  public toJSON() {
    return { x: this.x, y: this.y };
  }

  get id() {
    return this.format();
  }

  public static create(coord: Coord) {
    return new CoordImpl(coord);
  }

  public static fromIndex(index: ContinentIndex) {
    const size = NIL.world.refs().continentSize.value;
    const x = index % size;
    const y = index / size;
    return CoordImpl.create({ x, y });
  }

  public static toIndex(coord: Coord) {
    const size = NIL.world.refs().continentSize.value;
    return coord.y * size + coord.x;
  }

  private static readonly intl = new Intl.NumberFormat('default', {
    maximumFractionDigits: 0,
    minimumIntegerDigits: 3,
    style: 'decimal',
    useGrouping: false,
  });
}
