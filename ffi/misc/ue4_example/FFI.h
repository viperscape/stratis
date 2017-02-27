// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "GameFramework/Actor.h"
#include "FFI.generated.h"

UCLASS()
class STRATIS_API AFFI : public AActor
{
	GENERATED_BODY()
	
public:	
	// Sets default values for this actor's properties
	AFFI();
	void *v_dllHandle;

protected:
	// Called when the game starts or when spawned
	virtual void BeginPlay() override;

	bool importNewClient();
	bool importDropClient();
	bool importClientConnect();

public:	
	// Called every frame
	virtual void Tick(float DeltaTime) override;

	
	
};
