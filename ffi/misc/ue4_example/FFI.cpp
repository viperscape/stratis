// Fill out your copyright notice in the Description page of Project Settings.

#include "stratis.h"
#include "FFI.h"

typedef struct client_s client_t;
typedef client_t* (*_client)(void);
_client m_client;

typedef uint8_t u8;
typedef u8(*_client_drop)(client_t *);
_client_drop m_client_drop;

typedef u8(*_client_connect)(client_t *, const char*);
_client_connect m_client_connect;


// Sets default values
AFFI::AFFI()
{
 	// Set this actor to call Tick() every frame.  You can turn this off to improve performance if you don't need it.
	PrimaryActorTick.bCanEverTick = true;
}

// Called when the game starts or when spawned
void AFFI::BeginPlay()
{
	Super::BeginPlay();
	
	FString ffi = "stratis_ffi";
	FString filePath = *FPaths::GamePluginsDir() + ffi;
	v_dllHandle = FPlatformProcess::GetDllHandle(*filePath);
	if (v_dllHandle == NULL)
	{
		UE_LOG(LogLoad, Error, TEXT("unable to load ffi dll"));
		return;
	}

	UE_LOG(LogLoad, Display, TEXT("loaded ffi dll"));

	if (importNewClient() && 
		importDropClient() &&
		importClientConnect()) {
		client_t *c = m_client();
		UE_LOG(LogLoad, Display, TEXT("client new!"));

		u8 is_conn = m_client_connect(c, "127.0.0.1:9996");
		UE_LOG(LogLoad, Display, TEXT("%i"), is_conn);

		UE_LOG(LogLoad, Display, TEXT("%i"), m_client_drop(c));
	}
	else { UE_LOG(LogLoad, Error, TEXT("unable to import ffi proc")); }
}

bool AFFI::importNewClient() 
{
	m_client = NULL;
	FString procName = "new_client";
	m_client = (_client)FPlatformProcess::GetDllExport(v_dllHandle, *procName);

	return (m_client != NULL);
}

bool AFFI::importDropClient()
{
	m_client_drop = NULL;
	FString procName = "drop_client";
	m_client_drop = (_client_drop)FPlatformProcess::GetDllExport(v_dllHandle, *procName);

	return (m_client_drop != NULL);
}

bool AFFI::importClientConnect()
{
	m_client_connect = NULL;
	FString procName = "client_connect";
	m_client_connect = (_client_connect)FPlatformProcess::GetDllExport(v_dllHandle, *procName);

	return (m_client_connect != NULL);
}

// Called every frame
void AFFI::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

}

