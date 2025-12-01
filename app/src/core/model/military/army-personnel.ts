// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

import { SquadImpl } from './squad';

export class ArmyPersonnelImpl implements ArmyPersonnel {
  public readonly archer: SquadImpl;
  public readonly axeman: SquadImpl;
  public readonly heavyCavalry: SquadImpl;
  public readonly lightCavalry: SquadImpl;
  public readonly pikeman: SquadImpl;
  public readonly swordsman: SquadImpl;

  private constructor(personnel: ArmyPersonnel) {
    this.archer = SquadImpl.create(personnel.archer);
    this.axeman = SquadImpl.create(personnel.axeman);
    this.heavyCavalry = SquadImpl.create(personnel.heavyCavalry);
    this.lightCavalry = SquadImpl.create(personnel.lightCavalry);
    this.pikeman = SquadImpl.create(personnel.pikeman);
    this.swordsman = SquadImpl.create(personnel.swordsman);
  }

  public *[Symbol.iterator]() {
    yield* this.getSquads();
  }

  public getSquads() {
    return [
      this.archer,
      this.axeman,
      this.heavyCavalry,
      this.lightCavalry,
      this.pikeman,
      this.swordsman,
    ];
  }

  public size() {
    return ArmyPersonnelImpl.size(this);
  }

  public isEmpty() {
    return (
      this.archer.isEmpty() &&
      this.axeman.isEmpty() &&
      this.heavyCavalry.isEmpty() &&
      this.lightCavalry.isEmpty() &&
      this.pikeman.isEmpty() &&
      this.swordsman.isEmpty()
    );
  }

  public add(personnel: ArmyPersonnel) {
    return ArmyPersonnelImpl.create({
      archer: this.archer.add(personnel.archer),
      axeman: this.axeman.add(personnel.axeman),
      pikeman: this.pikeman.add(personnel.pikeman),
      swordsman: this.swordsman.add(personnel.swordsman),
      heavyCavalry: this.heavyCavalry.add(personnel.heavyCavalry),
      lightCavalry: this.lightCavalry.add(personnel.lightCavalry),
    });
  }

  public sub(personnel: ArmyPersonnel) {
    return ArmyPersonnelImpl.create({
      archer: this.archer.sub(personnel.archer),
      axeman: this.axeman.sub(personnel.axeman),
      pikeman: this.pikeman.sub(personnel.pikeman),
      swordsman: this.swordsman.sub(personnel.swordsman),
      heavyCavalry: this.heavyCavalry.sub(personnel.heavyCavalry),
      lightCavalry: this.lightCavalry.sub(personnel.lightCavalry),
    });
  }

  public static create(personnel: ArmyPersonnel) {
    if (personnel instanceof ArmyPersonnelImpl) {
      return personnel;
    }

    return new ArmyPersonnelImpl(personnel);
  }

  public static createEmpty() {
    return this.create({
      archer: SquadImpl.createEmpty('archer'),
      axeman: SquadImpl.createEmpty('axeman'),
      pikeman: SquadImpl.createEmpty('pikeman'),
      swordsman: SquadImpl.createEmpty('swordsman'),
      heavyCavalry: SquadImpl.createEmpty('heavy-cavalry'),
      lightCavalry: SquadImpl.createEmpty('light-cavalry'),
    });
  }

  public static size(personnel: ArmyPersonnel): ArmyPersonnelSize {
    return {
      archer: personnel.archer.size,
      axeman: personnel.axeman.size,
      heavyCavalry: personnel.heavyCavalry.size,
      lightCavalry: personnel.lightCavalry.size,
      pikeman: personnel.pikeman.size,
      swordsman: personnel.swordsman.size,
    };
  }
}
