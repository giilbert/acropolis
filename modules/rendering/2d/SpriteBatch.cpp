#include "rendering/2d/SpriteBatch.h"

#include <iostream>

using giz::SpriteBatch;

// const float vertices[] = {
//     0.5, 0.5, 0.0, 1.0,
//     -0.5, 0.5, 0.0, 1.0,
//     -0.5, -0.5, 0.0, 1.0,
//     0.5, -0.5, 0.0, 1.0};

// 3 0
// 2 1
const float vertices[] = {
    0.5, 0.5, 0.0, 1.0,
    0.5, -0.5, 0.0, 1.0,
    -0.5, -0.5, 0.0, 1.0,
    -0.5, 0.5, 0.0, 1.0};
const unsigned int indices[] = {
    0, 2, 1,
    0, 3, 2};

SpriteBatch::SpriteBatch()
{
    glGenVertexArrays(1, &m_VaoId);
    std::cout << "vao id: " << m_VaoId << "\n";
    glBindVertexArray(m_VaoId);

    unsigned int vertexBuffer, indexBuffer;
    glCreateBuffers(1, &vertexBuffer);
    glCreateBuffers(1, &indexBuffer);

    glBindBuffer(GL_ARRAY_BUFFER, vertexBuffer);
    glBufferData(GL_ARRAY_BUFFER, sizeof(vertices), vertices, GL_STATIC_DRAW);

    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, indexBuffer);
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(indices), indices, GL_STATIC_DRAW);

    glVertexAttribPointer(0, 4, GL_FLOAT, GL_FALSE, 4 * sizeof(float), (void *)0);
    glEnableVertexAttribArray(0);
}

void SpriteBatch::Draw()
{
    glBindVertexArray(m_VaoId);
    glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_INT, 0);
}