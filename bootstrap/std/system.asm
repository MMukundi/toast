%define Syscode.Exit   0x2000001
%define Syscode.Fork   0x2000002
%define Syscode.Read   0x2000003
%define Syscode.Write  0x2000004
%define Syscode.Open   0x2000005
%define Syscode.Close  0x2000006
%define Syscode.Wait4  0x2000007
%define Syscode.Exec   0x200003b
%define Syscode.Vfork  0x2000042
%define Syscode.Socket 0x2000097

%macro __Toast__Make__Syscall__ 1
	mov rax, %1
	syscall
%endmacro

%macro Syscall.Exit 0
	__Toast__Make__Syscall__ Syscode.Exit
%endmacro
%macro Syscall.Read 0
	__Toast__Make__Syscall__ Syscode.Read
%endmacro
%macro Syscall.Write 0
	__Toast__Make__Syscall__ Syscode.Write
%endmacro
%macro Syscall.Open 0
	__Toast__Make__Syscall__ Syscode.Open
%endmacro
%macro Syscall.Close 0
	__Toast__Make__Syscall__ Syscode.Close
%endmacro
%macro Syscall.Exec 0
	__Toast__Make__Syscall__ Syscode.Exec
%endmacro
%macro Syscall.Fork 0
	__Toast__Make__Syscall__ Syscode.Fork
%endmacro

; %macro InitSyscalls 0-*
; 	%rep %0/2
; 		%define CodeName %1
; 		%define Syscode.%[CodeName] %2

; 		%macro Syscall.%[CodeName] 0
; 			mov rax, Syscode.%[CodeName]
; 			syscall
; 		%endmacro

; 		%rotate 2
; 	%endrep
; %endmacro

; InitSyscalls \
; Exit   0x2000001 \
; Fork   0x2000002 \
; Read   0x2000003 \
; Write  0x2000004 \
; Open   0x2000005 \
; Close  0x2000006 \
; Wait4  0x2000007 \
; Exec   0x200003b \
; Vfork  0x2000042 \
; Socket 0x2000097
