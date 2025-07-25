// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

interface Academy extends Building {
  readonly recruitQueue: AcademyRecruitQueue;
}

interface AcademyRecruitQueue {
  readonly orders: readonly AcademyRecruitOrder[];
}

interface AcademyRecruitOrder {
  readonly id: AcademyRecruitOrderId;
  readonly unit: UnitId;
  readonly resources: Resources;
  readonly workforce: number;
  readonly state: AcademyRecruitOrderState;
}

type AcademyRecruitOrderId = string;

type AcademyRecruitOrderState = AcademyRecruitOrderStatePending | AcademyRecruitOrderStateDone;

interface AcademyRecruitOrderStatePending {
  readonly kind: 'pending';
  readonly workforce: number;
}

interface AcademyRecruitOrderStateDone {
  readonly kind: 'done';
}

interface AcademyRecruitOrderRequest {
  readonly coord: Coord;
  readonly unit: UnitId;
  readonly chunks: number;
}

type AcademyRecruitCatalog = {
  readonly [U in keyof ArmyAcademyPersonnel]: AcademyRecruitCatalogEntry;
};

type AcademyRecruitCatalogEntry =
  | AcademyRecruitCatalogEntryAvailable
  | AcademyRecruitCatalogEntryUnmet;

interface AcademyRecruitCatalogEntryAvailable {
  readonly kind: 'available';
  readonly recipe: AcademyRecruitCatalogRecipe;
}

interface AcademyRecruitCatalogEntryUnmet {
  readonly kind: 'unmet';
  readonly requirements: InfrastructureRequirements;
}

interface AcademyRecruitCatalogRecipe {
  readonly resources: Resources;
  readonly maintenance: number;
  readonly workforce: number;
  readonly requirements: InfrastructureRequirements;
}
