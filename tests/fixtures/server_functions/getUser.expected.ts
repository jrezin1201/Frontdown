export async function getUser(id: string) {
  return await db.users.findUnique({ where: { id } });
}
