	.text
	.file	"main.c"
	.globl	main                    # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r12
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	subq	$40, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -40
	.cfi_offset %r12, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	cmpl	$2, %edi
	jne	.LBB0_1
# %bb.2:
	movq	8(%rsi), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	callq	strtol
	movslq	%eax, %r14
	jmp	.LBB0_3
.LBB0_1:
	movl	$32, %r14d
.LBB0_3:
	leaq	(,%r14,4), %rbx
	movq	%rbx, %rdi
	callq	malloc
	movq	%rax, %r15
	movq	%rbx, %rdi
	callq	malloc
	movq	%rax, %rbx
	xorl	%r12d, %r12d
	movl	$.L.str, %edi
	xorl	%eax, %eax
	movq	%r14, %rsi
	callq	printf
	leaq	24(%rsp), %rdi
	movl	$1, %esi
	callq	timespec_get
	testl	%r14d, %r14d
	jle	.LBB0_10
# %bb.4:
	movl	%r14d, %eax
	andl	$7, %r14d
	movq	%rax, %rcx
	subq	%r14, %rcx
	.p2align	4, 0x90
.LBB0_5:                                # =>This Loop Header: Depth=1
                                        #     Child Loop BB0_6 Depth 2
                                        #     Child Loop BB0_8 Depth 2
	xorl	%edx, %edx
	cmpq	$8, %rax
	jb	.LBB0_8
	.p2align	4, 0x90
.LBB0_6:                                #   Parent Loop BB0_5 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	movups	(%r15,%rdx,4), %xmm0
	movups	16(%r15,%rdx,4), %xmm1
	addps	%xmm0, %xmm0
	addps	%xmm1, %xmm1
	movups	(%rbx,%rdx,4), %xmm2
	addps	%xmm0, %xmm2
	movups	16(%rbx,%rdx,4), %xmm0
	addps	%xmm1, %xmm0
	movups	%xmm2, (%rbx,%rdx,4)
	movups	%xmm0, 16(%rbx,%rdx,4)
	addq	$8, %rdx
	cmpq	%rdx, %rcx
	jne	.LBB0_6
# %bb.7:                                #   in Loop: Header=BB0_5 Depth=1
	movq	%rcx, %rdx
	testq	%r14, %r14
	je	.LBB0_9
	.p2align	4, 0x90
.LBB0_8:                                #   Parent Loop BB0_5 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	movss	(%r15,%rdx,4), %xmm0    # xmm0 = mem[0],zero,zero,zero
	addss	%xmm0, %xmm0
	addss	(%rbx,%rdx,4), %xmm0
	movss	%xmm0, (%rbx,%rdx,4)
	addq	$1, %rdx
	cmpq	%rdx, %rax
	jne	.LBB0_8
.LBB0_9:                                #   in Loop: Header=BB0_5 Depth=1
	addl	$1, %r12d
	cmpl	$100000, %r12d          # imm = 0x186A0
	jne	.LBB0_5
.LBB0_10:
	leaq	8(%rsp), %rdi
	movl	$1, %esi
	callq	timespec_get
	movq	8(%rsp), %rdi
	movq	24(%rsp), %rsi
	callq	difftime
	cvttsd2si	%xmm0, %rcx
	movq	16(%rsp), %rax
	subq	32(%rsp), %rax
	imulq	$1000, %rcx, %rcx       # imm = 0x3E8
	movabsq	$4835703278458516699, %rdx # imm = 0x431BDE82D7B634DB
	imulq	%rdx
	movq	%rdx, %rax
	shrq	$63, %rax
	sarq	$18, %rdx
	leaq	(%rdx,%rax), %rsi
	addq	%rcx, %rsi
	movl	$.L.str.1, %edi
	xorl	%eax, %eax
	callq	printf
	movl	$1, %eax
	addq	$40, %rsp
	popq	%rbx
	popq	%r12
	popq	%r14
	popq	%r15
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.globl	saxpy                   # -- Begin function saxpy
	.p2align	4, 0x90
	.type	saxpy,@function
saxpy:                                  # @saxpy
	.cfi_startproc
# %bb.0:
	testl	%edi, %edi
	jle	.LBB1_11
# %bb.1:
	movl	%edi, %r9d
	cmpl	$7, %edi
	ja	.LBB1_3
# %bb.2:
	xorl	%ecx, %ecx
	jmp	.LBB1_10
.LBB1_3:
	movl	%r9d, %ecx
	andl	$-8, %ecx
	movaps	%xmm0, %xmm1
	shufps	$0, %xmm1, %xmm1        # xmm1 = xmm1[0,0,0,0]
	leaq	-8(%rcx), %rax
	movq	%rax, %rdi
	shrq	$3, %rdi
	leal	1(%rdi), %r8d
	andl	$1, %r8d
	testq	%rax, %rax
	je	.LBB1_4
# %bb.5:
	leaq	-1(%r8), %rax
	subq	%rdi, %rax
	xorl	%edi, %edi
	.p2align	4, 0x90
.LBB1_6:                                # =>This Inner Loop Header: Depth=1
	movups	(%rsi,%rdi,4), %xmm2
	movups	16(%rsi,%rdi,4), %xmm3
	mulps	%xmm1, %xmm2
	mulps	%xmm1, %xmm3
	movups	(%rdx,%rdi,4), %xmm4
	addps	%xmm2, %xmm4
	movups	16(%rdx,%rdi,4), %xmm2
	addps	%xmm3, %xmm2
	movups	32(%rdx,%rdi,4), %xmm3
	movups	48(%rdx,%rdi,4), %xmm5
	movups	%xmm4, (%rdx,%rdi,4)
	movups	%xmm2, 16(%rdx,%rdi,4)
	movups	32(%rsi,%rdi,4), %xmm2
	movups	48(%rsi,%rdi,4), %xmm4
	mulps	%xmm1, %xmm2
	addps	%xmm3, %xmm2
	mulps	%xmm1, %xmm4
	addps	%xmm5, %xmm4
	movups	%xmm2, 32(%rdx,%rdi,4)
	movups	%xmm4, 48(%rdx,%rdi,4)
	addq	$16, %rdi
	addq	$2, %rax
	jne	.LBB1_6
# %bb.7:
	testq	%r8, %r8
	je	.LBB1_9
.LBB1_8:
	movups	(%rsi,%rdi,4), %xmm2
	movups	16(%rsi,%rdi,4), %xmm3
	mulps	%xmm1, %xmm2
	mulps	%xmm1, %xmm3
	movups	(%rdx,%rdi,4), %xmm1
	addps	%xmm2, %xmm1
	movups	16(%rdx,%rdi,4), %xmm2
	addps	%xmm3, %xmm2
	movups	%xmm1, (%rdx,%rdi,4)
	movups	%xmm2, 16(%rdx,%rdi,4)
.LBB1_9:
	cmpq	%r9, %rcx
	je	.LBB1_11
	.p2align	4, 0x90
.LBB1_10:                               # =>This Inner Loop Header: Depth=1
	movss	(%rsi,%rcx,4), %xmm1    # xmm1 = mem[0],zero,zero,zero
	mulss	%xmm0, %xmm1
	addss	(%rdx,%rcx,4), %xmm1
	movss	%xmm1, (%rdx,%rcx,4)
	addq	$1, %rcx
	cmpq	%rcx, %r9
	jne	.LBB1_10
.LBB1_11:
	retq
.LBB1_4:
	xorl	%edi, %edi
	testq	%r8, %r8
	jne	.LBB1_8
	jmp	.LBB1_9
.Lfunc_end1:
	.size	saxpy, .Lfunc_end1-saxpy
	.cfi_endproc
                                        # -- End function
	.type	.L.str,@object          # @.str
	.section	.rodata.str1.1,"aMS",@progbits,1
.L.str:
	.asciz	"Running saxpy on arrays of size %lu...\n"
	.size	.L.str, 40

	.type	.L.str.1,@object        # @.str.1
.L.str.1:
	.asciz	"Finished saxpy. Time taken: %lu milliseconds\n"
	.size	.L.str.1, 46


	.ident	"clang version 6.0.0-1ubuntu2 (tags/RELEASE_600/final)"
	.section	".note.GNU-stack","",@progbits
